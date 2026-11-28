use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    CapabilitySet,
    FacadeError,
    ImplementationName,
    StoreDescription,
    StoreIri,
    StoreLabels,
    StoreSlug,
};

/// An operator's decision that one implementation, with configuration `C`,
/// serves a Store IRI.
///
/// The binding holds what callers will see of the store (its IRI, slug and
/// labels) next to what they never see (the implementation and its
/// configuration). A [`crate::Binder`] adds the capabilities it actually
/// supports through [`StoreBinding::describe`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreBinding<C> {
    store: StoreIri,
    slug: StoreSlug,
    labels: StoreLabels,
    implementation: ImplementationName,
    config: C,
}

impl<C> StoreBinding<C> {
    /// Binds `store` to `implementation` configured by `config`.
    ///
    /// # Errors
    ///
    /// [`FacadeError::MissingLabel`] when `labels` is empty.
    pub fn try_new(
        store: StoreIri,
        slug: StoreSlug,
        labels: StoreLabels,
        implementation: ImplementationName,
        config: C,
    ) -> Result<Self, FacadeError> {
        if labels.is_empty() {
            return Err(FacadeError::MissingLabel);
        }
        Ok(Self {
            store,
            slug,
            labels,
            implementation,
            config,
        })
    }

    /// The bound Store IRI.
    #[must_use]
    pub const fn store(&self) -> &StoreIri {
        &self.store
    }

    /// The selected implementation.
    #[must_use]
    pub const fn implementation(&self) -> &ImplementationName {
        &self.implementation
    }

    /// The implementation's configuration.
    #[must_use]
    pub const fn config(&self) -> &C {
        &self.config
    }

    /// Confirms this binding selects `expected`.
    ///
    /// # Errors
    ///
    /// [`FacadeError::BindingMismatch`] for any other implementation.
    pub fn expect_implementation(&self, expected: &ImplementationName) -> Result<(), FacadeError> {
        if &self.implementation == expected {
            Ok(())
        } else {
            Err(FacadeError::BindingMismatch {
                expected: Box::new(expected.clone()),
                actual: Box::new(self.implementation.clone()),
            })
        }
    }

    /// The caller-facing description of the bound store with `capabilities`,
    /// which names nothing of the implementation or its configuration.
    ///
    /// # Errors
    ///
    /// [`FacadeError::MissingLabel`] when the binding has no label.
    pub fn describe(&self, capabilities: CapabilitySet) -> Result<StoreDescription, FacadeError> {
        StoreDescription::try_new(
            self.store.clone(),
            self.slug.clone(),
            self.labels.clone(),
            capabilities,
        )
    }

    /// Splits off the configuration, keeping the caller-facing description
    /// with `capabilities`.
    ///
    /// # Errors
    ///
    /// As for [`StoreBinding::describe`].
    pub fn into_parts(
        self,
        capabilities: CapabilitySet,
    ) -> Result<(StoreDescription, C), FacadeError> {
        let description =
            StoreDescription::try_new(self.store, self.slug, self.labels, capabilities)?;
        Ok((description, self.config))
    }
}
