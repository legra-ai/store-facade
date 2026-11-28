use crate::{
    FacadeError,
    ImplementationName,
    QueryStore,
    StoreBinding,
};

/// Opens stores of one implementation from operator bindings.
///
/// A binder declares only the capabilities its store actually supports; the
/// store-facade contract suite holds every declared capability to its tests.
pub trait Binder {
    /// The implementation's configuration.
    type Config;
    /// The store a binding opens.
    type Store: QueryStore;

    /// The implementation this binder serves.
    fn implementation(&self) -> ImplementationName;

    /// Opens the store `binding` selects; implementations call
    /// [`Binder::check`] first.
    ///
    /// # Errors
    ///
    /// [`FacadeError::BindingMismatch`] for a binding of another
    /// implementation, or any error opening the store.
    fn bind(&self, binding: StoreBinding<Self::Config>) -> Result<Self::Store, FacadeError>;

    /// Confirms `binding` selects this binder's implementation.
    ///
    /// # Errors
    ///
    /// [`FacadeError::BindingMismatch`] for any other implementation.
    fn check(&self, binding: &StoreBinding<Self::Config>) -> Result<(), FacadeError> {
        binding.expect_implementation(&self.implementation())
    }
}
