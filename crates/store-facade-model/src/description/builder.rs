use rdf_structural_model::LanguageTag;

use crate::{
    Capability,
    CapabilitySet,
    FacadeError,
    LabelText,
    StoreDescription,
    StoreIri,
    StoreLabel,
    StoreLabels,
    StoreSlug,
};

/// Builds a [`StoreDescription`]; the first invalid input fails
/// [`StoreDescriptionBuilder::build`].
#[derive(Debug, Clone)]
#[must_use]
pub struct StoreDescriptionBuilder {
    iri: StoreIri,
    slug: StoreSlug,
    labels: StoreLabels,
    capabilities: CapabilitySet,
    error: Option<FacadeError>,
}

impl StoreDescriptionBuilder {
    pub(super) const fn new(iri: StoreIri, slug: StoreSlug) -> Self {
        Self {
            iri,
            slug,
            labels: StoreLabels::new(),
            capabilities: CapabilitySet::new(),
            error: None,
        }
    }

    /// Adds a label in `language`.
    pub fn label(mut self, language: LanguageTag, text: impl Into<String>) -> Self {
        if self.error.is_none() {
            let added = LabelText::try_new(text)
                .and_then(|text| self.labels.insert(StoreLabel::new(language, text)));
            self.error = added.err();
        }
        self
    }

    /// Adds a capability.
    pub fn capability(mut self, capability: impl Into<Capability>) -> Self {
        if self.error.is_none() {
            self.error = self.capabilities.insert(capability.into()).err();
        }
        self
    }

    /// Adds capabilities.
    pub fn capabilities<C: Into<Capability>>(
        self,
        capabilities: impl IntoIterator<Item = C>,
    ) -> Self {
        capabilities.into_iter().fold(self, Self::capability)
    }

    /// Finishes the description.
    ///
    /// # Errors
    ///
    /// The first error any input raised, or [`FacadeError::MissingLabel`].
    pub fn build(self) -> Result<StoreDescription, FacadeError> {
        if let Some(error) = self.error {
            return Err(error);
        }
        StoreDescription::try_new(self.iri, self.slug, self.labels, self.capabilities)
    }
}
