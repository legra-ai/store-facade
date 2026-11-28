use core::fmt;
use std::io::Read;

use oxigraph::io::{
    RdfFormat,
    RdfParser,
};
use oxigraph::sparql::SparqlEvaluator;
use oxigraph::store::Store;
use store_facade_model::vocabulary::{
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
};
use store_facade_model::{
    FacadeError,
    QueryRequest,
    QueryStore,
    StoreDescription,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::error::ErrorTranslation;
use crate::execution::{
    Interruption,
    ItemSink,
    Producer,
    UpdateRun,
};
use crate::{
    OxigraphConfig,
    QueryProgress,
    ResultBuffer,
    ResultStream,
};

/// A store-facade store answering SPARQL from an embedded Oxigraph store.
///
/// Queries evaluate on blocking threads and stream through a bounded buffer;
/// updates apply atomically. Open one with [`crate::OxigraphBinder`].
#[derive(Clone)]
pub struct OxigraphStore {
    description: StoreDescription,
    store: Store,
    union_default_graph: bool,
    buffer: ResultBuffer,
}

impl OxigraphStore {
    pub(crate) fn open(
        description: StoreDescription,
        config: &OxigraphConfig,
    ) -> Result<Self, FacadeError> {
        Ok(Self {
            description,
            store: config.storage().open()?,
            union_default_graph: config.union_default_graph(),
            buffer: config.result_buffer(),
        })
    }

    /// Loads RDF from `reader` on a blocking thread, streaming it in batches
    /// so memory stays bounded whatever the input size. Operator side: it is
    /// not part of the caller contract.
    ///
    /// # Errors
    ///
    /// [`FacadeError::Backend`] for unparsable data or a storage failure.
    pub async fn load(
        &self,
        format: RdfFormat,
        reader: impl Read + Send + 'static,
    ) -> Result<(), FacadeError> {
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || {
            let mut loader = store.bulk_loader();
            loader
                .load_from_reader(RdfParser::from_format(format), reader)
                .map_err(ErrorTranslation::from_load)?;
            loader.commit().map_err(ErrorTranslation::from_storage)
        })
        .await
        .map_err(|_| FacadeError::backend("the load task failed"))?
    }

    fn start_query(
        &self,
        request: &QueryRequest,
        interruption: Interruption,
    ) -> Result<ResultStream, FacadeError> {
        let mut query = SparqlEvaluator::new()
            .with_cancellation_token(interruption.evaluation_token())
            .parse_query(request.text().as_str())
            .map_err(|error| ErrorTranslation::from_syntax(&error))?;
        if self.union_default_graph {
            query.dataset_mut().set_default_graph_as_union();
        }
        let (sender, receiver) = mpsc::channel(self.buffer.items());
        let progress = QueryProgress::new();
        let release = interruption.release_on_drop();
        let timed_out = interruption.timed_out();
        let producer = Producer::new(
            self.store.clone(),
            query,
            ItemSink::new(sender, interruption, progress.clone()),
        );
        tokio::task::spawn_blocking(move || producer.run());
        Ok(ResultStream::new(receiver, progress, timed_out, release))
    }

    async fn apply_update(
        &self,
        request: &QueryRequest,
        interruption: Interruption,
    ) -> Result<ResultStream, FacadeError> {
        let update = SparqlEvaluator::new()
            .with_cancellation_token(interruption.evaluation_token())
            .parse_update(request.text().as_str())
            .map_err(|error| ErrorTranslation::from_syntax(&error))?;
        let run = UpdateRun::new(self.store.clone(), update, interruption);
        tokio::task::spawn_blocking(move || run.run())
            .await
            .map_err(|_| FacadeError::backend("the update task failed"))??;
        Ok(ResultStream::empty())
    }
}

impl fmt::Debug for OxigraphStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OxigraphStore")
            .field("description", &self.description)
            .field("union_default_graph", &self.union_default_graph)
            .field("buffer", &self.buffer)
            .finish_non_exhaustive()
    }
}

impl QueryStore for OxigraphStore {
    type Item = crate::ResultItem;
    type Stream = ResultStream;

    fn description(&self) -> &StoreDescription {
        &self.description
    }

    async fn execute(
        &self,
        request: QueryRequest,
        cancellation: CancellationToken,
    ) -> Result<ResultStream, FacadeError> {
        let interruption = Interruption::start(cancellation, request.timeout());
        let language = request.language().capability();
        if *language == SPARQL11_QUERY.capability() || *language == SPARQL12_QUERY.capability() {
            self.start_query(&request, interruption)
        } else if *language == SPARQL11_UPDATE.capability()
            || *language == SPARQL12_UPDATE.capability()
        {
            self.apply_update(&request, interruption).await
        } else {
            Err(FacadeError::backend(format!(
                "the store has no evaluator for <{language}>"
            )))
        }
    }
}
