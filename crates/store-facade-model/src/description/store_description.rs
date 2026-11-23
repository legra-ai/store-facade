use serde::{
    Deserialize,
    Serialize,
};

use super::wire::DescriptionFields;
use crate::{
    CapabilitySet,
    FacadeError,
    RequiredCapabilities,
    StoreDescriptionBuilder,
    StoreIri,
    StoreLabels,
    StoreSlug,
};

/// What a caller knows about one store: its IRI, slug, labels and
/// capabilities, and nothing about the technology behind it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "DescriptionFields")]
pub struct StoreDescription {
    iri: StoreIri,
    slug: StoreSlug,
    labels: StoreLabels,
    capabilities: CapabilitySet,
}

impl StoreDescription {
    /// Assembles a description.
    ///
    /// # Errors
    ///
    /// [`FacadeError::MissingLabel`] when `labels` is empty.
    pub fn try_new(
        iri: StoreIri,
        slug: StoreSlug,
        labels: StoreLabels,
        capabilities: CapabilitySet,
    ) -> Result<Self, FacadeError> {
        if labels.is_empty() {
            return Err(FacadeError::MissingLabel);
        }
        Ok(Self {
            iri,
            slug,
            labels,
            capabilities,
        })
    }

    /// Starts building a description of `iri`.
    pub fn builder(iri: StoreIri, slug: StoreSlug) -> StoreDescriptionBuilder {
        StoreDescriptionBuilder::new(iri, slug)
    }

    /// The Store IRI.
    #[must_use]
    pub const fn iri(&self) -> &StoreIri {
        &self.iri
    }

    /// The slug.
    #[must_use]
    pub const fn slug(&self) -> &StoreSlug {
        &self.slug
    }

    /// The labels, never empty.
    #[must_use]
    pub const fn labels(&self) -> &StoreLabels {
        &self.labels
    }

    /// The capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> &CapabilitySet {
        &self.capabilities
    }

    /// Confirms the store has every required capability.
    ///
    /// # Errors
    ///
    /// [`FacadeError::MissingCapabilities`] naming every absent capability.
    pub fn require(&self, required: &RequiredCapabilities) -> Result<(), FacadeError> {
        self.capabilities.require(required)
    }
}
