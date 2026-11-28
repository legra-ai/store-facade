use store_facade_model::{
    Binder,
    FacadeError,
    ImplementationName,
    StoreBinding,
};

use crate::{
    OxigraphConfig,
    OxigraphStore,
};

/// Opens [`OxigraphStore`]s from bindings that select `oxigraph`.
#[derive(Debug, Clone, Copy, Default)]
pub struct OxigraphBinder;

impl OxigraphBinder {
    /// The implementation name bindings use to select this binder.
    pub const IMPLEMENTATION: &'static str = "oxigraph";
}

impl Binder for OxigraphBinder {
    type Config = OxigraphConfig;
    type Store = OxigraphStore;

    fn implementation(&self) -> ImplementationName {
        ImplementationName::try_new(Self::IMPLEMENTATION).unwrap_or_else(|_| unreachable!())
    }

    /// Opens the store, creating an on-disk directory when absent.
    ///
    /// Opening blocks the calling thread on storage I/O; call it while
    /// starting up, or from a blocking context.
    fn bind(&self, binding: StoreBinding<OxigraphConfig>) -> Result<OxigraphStore, FacadeError> {
        self.check(&binding)?;
        let capabilities = binding.config().capabilities();
        let (description, config) = binding.into_parts(capabilities)?;
        OxigraphStore::open(description, &config)
    }
}
