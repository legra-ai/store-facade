use futures_util::StreamExt;
use store_facade_model::vocabulary::{
    CapabilityTerm,
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
};
use store_facade_model::{
    FacadeError,
    QueryLanguage,
    QueryRequest,
    QueryStore,
    QueryText,
    QueryTimeout,
};
use tokio_util::sync::CancellationToken;

use crate::{
    ContractBinding,
    Observation,
    SeedQuads,
};

/// One store under test and the binding that opened it.
pub struct Probe<'binding, B: ContractBinding> {
    binding: &'binding B,
    store: B::Store,
}

impl<'binding, B: ContractBinding> Probe<'binding, B> {
    /// The most items a contract query may return.
    pub const MAX_OBSERVATIONS: usize = 1024;

    /// Opens a fresh store of `binding`.
    ///
    /// # Panics
    ///
    /// When the binding cannot open a store.
    pub async fn open(binding: &'binding B) -> Self {
        let store = binding.open().await.expect("the binding opens a store");
        Self { binding, store }
    }

    /// The store under test.
    pub const fn store(&self) -> &B::Store {
        &self.store
    }

    /// Whether the store declares `capability`.
    pub fn declares(&self, capability: CapabilityTerm) -> bool {
        self.store
            .description()
            .capabilities()
            .contains(&capability.capability())
    }

    /// Seeds the store.
    ///
    /// # Panics
    ///
    /// When seeding fails.
    pub async fn seed(&self, quads: &SeedQuads) {
        self.binding
            .seed(&self.store, quads)
            .await
            .expect("the binding seeds its store");
    }

    /// A request in `language` addressed to this store.
    ///
    /// # Panics
    ///
    /// When `language` is not a language or `text` is blank.
    pub fn request(&self, language: CapabilityTerm, text: &str) -> QueryRequest {
        QueryRequest::new(
            self.store.description().iri().clone(),
            QueryLanguage::try_from(language).expect("a language capability"),
            QueryText::try_new(text).expect("a non-empty query"),
        )
    }

    /// A request in `language` that ends after `millis` milliseconds.
    ///
    /// # Panics
    ///
    /// As for [`Probe::request`], or for an out-of-range timeout.
    pub fn timed_request(&self, language: CapabilityTerm, text: &str, millis: u64) -> QueryRequest {
        self.request(language, text)
            .with_timeout(QueryTimeout::try_from_millis(millis).expect("an in-range timeout"))
    }

    /// Runs `request` to completion and renders every item; the first error,
    /// whether starting or streaming, is the outcome.
    ///
    /// # Errors
    ///
    /// The store's first error.
    ///
    /// # Panics
    ///
    /// Past [`Probe::MAX_OBSERVATIONS`] items.
    pub async fn run(&self, request: QueryRequest) -> Result<Vec<Observation>, FacadeError> {
        let mut stream = self.store.query(request, CancellationToken::new()).await?;
        // bounded: at most `MAX_OBSERVATIONS` items.
        let mut observations = Vec::new();
        while let Some(item) = stream.next().await {
            assert!(
                observations.len() < Self::MAX_OBSERVATIONS,
                "a contract query returned more than {} items",
                Self::MAX_OBSERVATIONS
            );
            observations.push(B::observe(&item?));
        }
        Ok(observations)
    }

    /// Runs an `ASK` query in `language`.
    ///
    /// # Panics
    ///
    /// When the query fails or does not answer one boolean.
    pub async fn ask(&self, language: CapabilityTerm, text: &str) -> bool {
        let observations = self
            .run(self.request(language, text))
            .await
            .unwrap_or_else(|error| panic!("ASK failed: {error} [{}]", error.code()));
        match observations.as_slice() {
            [observation] => observation
                .as_boolean()
                .expect("an ASK query answers a boolean"),
            other => panic!("an ASK query answers exactly one boolean, got {other:?}"),
        }
    }

    /// Runs an update in `language`.
    ///
    /// # Panics
    ///
    /// When the update fails.
    pub async fn update(&self, language: CapabilityTerm, text: &str) {
        self.run(self.request(language, text))
            .await
            .unwrap_or_else(|error| panic!("update failed: {error} [{}]", error.code()));
    }
}

impl<B: ContractBinding> Probe<'_, B> {
    /// The query language checks observe through: SPARQL 1.2 when declared,
    /// else SPARQL 1.1.
    ///
    /// # Panics
    ///
    /// When the store declares neither, naming `capability`, which then has
    /// no observable behaviour.
    pub fn query_language(&self, capability: CapabilityTerm) -> CapabilityTerm {
        [SPARQL12_QUERY, SPARQL11_QUERY]
            .into_iter()
            .find(|language| self.declares(*language))
            .unwrap_or_else(|| {
                panic!(
                    "<{}> is only observable through a declared SPARQL query language",
                    capability.capability()
                )
            })
    }

    /// The update language checks write through: SPARQL 1.2 Update when
    /// declared, else SPARQL 1.1 Update.
    ///
    /// # Panics
    ///
    /// When the store declares neither, naming `capability`.
    pub fn update_language(&self, capability: CapabilityTerm) -> CapabilityTerm {
        [SPARQL12_UPDATE, SPARQL11_UPDATE]
            .into_iter()
            .find(|language| self.declares(*language))
            .unwrap_or_else(|| {
                panic!(
                    "<{}> is only observable through a declared SPARQL update language",
                    capability.capability()
                )
            })
    }

    /// Runs a `SELECT` query projecting `variable` and returns its terms in
    /// solution order.
    ///
    /// # Panics
    ///
    /// When the query fails or returns a non-solution.
    pub async fn select(
        &self,
        language: CapabilityTerm,
        text: &str,
        variable: &str,
    ) -> Vec<String> {
        let observations = self
            .run(self.request(language, text))
            .await
            .unwrap_or_else(|error| panic!("SELECT failed: {error} [{}]", error.code()));
        // bounded: at most `MAX_OBSERVATIONS` items.
        observations
            .iter()
            .map(|observation| {
                observation
                    .as_solution()
                    .and_then(|solution| solution.get(variable))
                    .map_or_else(
                        || panic!("a solution binding ?{variable}, got {observation:?}"),
                        |term| term.as_str().to_owned(),
                    )
            })
            .collect()
    }
}
