use core::future::Future;

use store_facade_model::{
    FacadeError,
    QueryStore,
};

use crate::{
    Observation,
    SeedQuads,
};

/// What a binding supplies to run the contract suite against its stores.
///
/// Implement it for a small test type; [`crate::contract_tests!`] constructs
/// one per test, so each test gets its own fresh store.
pub trait ContractBinding: Send + Sync {
    /// The store under test.
    type Store: QueryStore;

    /// Opens a fresh, empty store.
    fn open(&self) -> impl Future<Output = Result<Self::Store, FacadeError>> + Send;

    /// Loads `quads` into `store` through the operator side, never through a
    /// query language, so query checks do not depend on update support.
    fn seed(
        &self,
        store: &Self::Store,
        quads: &SeedQuads,
    ) -> impl Future<Output = Result<(), FacadeError>> + Send;

    /// Renders one result item.
    fn observe(item: &<Self::Store as QueryStore>::Item) -> Observation;
}
