use oxigraph::sparql::{
    PreparedSparqlQuery,
    QueryEvaluationError,
    QueryResults,
};
use oxigraph::store::Store;
use store_facade_model::FacadeError;
use tokio::sync::mpsc::Sender;

use super::Interruption;
use crate::error::ErrorTranslation;
use crate::{
    QueryProgress,
    ResultItem,
};

/// Evaluates one query on a blocking thread into an [`ItemSink`].
pub(crate) struct Producer {
    store: Store,
    query: PreparedSparqlQuery,
    sink: ItemSink,
}

impl Producer {
    pub(crate) const fn new(store: Store, query: PreparedSparqlQuery, sink: ItemSink) -> Self {
        Self { store, query, sink }
    }

    /// Runs to the end of the results, the first error, an interruption or
    /// the consumer's departure.
    pub(crate) fn run(self) {
        let Self { store, query, sink } = self;
        let _released = sink.interruption.release_on_drop();
        match query.on_store(&store).execute() {
            Err(error) => {
                sink.send(Err(ErrorTranslation::from_query(error, &sink.interruption)));
            }
            Ok(QueryResults::Boolean(answer)) => {
                sink.send(Ok(ResultItem::Boolean(answer)));
            }
            Ok(QueryResults::Solutions(solutions)) => {
                sink.drain(solutions.map(|solution| solution.map(ResultItem::Solution)));
            }
            Ok(QueryResults::Graph(triples)) => {
                sink.drain(triples.map(|triple| triple.map(ResultItem::Triple)));
            }
        }
        sink.progress.finish();
    }
}

/// The sending end of one query's bounded result channel.
///
/// Sending blocks while the channel is full (back-pressure). The sink checks
/// for an interruption before every item and stops as soon as the consumer
/// is gone, so it never holds more than one item outside the channel.
pub(crate) struct ItemSink {
    sender: Sender<Result<ResultItem, FacadeError>>,
    interruption: Interruption,
    progress: QueryProgress,
}

impl ItemSink {
    pub(crate) const fn new(
        sender: Sender<Result<ResultItem, FacadeError>>,
        interruption: Interruption,
        progress: QueryProgress,
    ) -> Self {
        Self {
            sender,
            interruption,
            progress,
        }
    }

    fn drain(&self, items: impl Iterator<Item = Result<ResultItem, QueryEvaluationError>>) {
        for item in items {
            if self.interruption.is_interrupted() {
                self.send(Err(self.interruption.error()));
                return;
            }
            let item =
                item.map_err(|error| ErrorTranslation::from_query(error, &self.interruption));
            let failed = item.is_err();
            if !self.send(item) || failed {
                return;
            }
        }
    }

    /// Sends one item, blocking while the channel is full; false once the
    /// consumer is gone.
    fn send(&self, item: Result<ResultItem, FacadeError>) -> bool {
        let sent = self.sender.blocking_send(item).is_ok();
        if sent {
            self.progress.record_item();
        }
        sent
    }
}
