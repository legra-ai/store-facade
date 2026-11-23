use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    CacheKey,
    FacadeError,
    QueryLanguage,
    QueryText,
    RequiredCapabilities,
    StoreDescription,
    StoreIri,
};

/// A language-tagged query addressed to one store.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryRequest {
    store: StoreIri,
    language: QueryLanguage,
    text: QueryText,
}

impl QueryRequest {
    /// A query for `store` in `language`.
    #[must_use]
    pub const fn new(store: StoreIri, language: QueryLanguage, text: QueryText) -> Self {
        Self {
            store,
            language,
            text,
        }
    }

    /// The addressed store.
    #[must_use]
    pub const fn store(&self) -> &StoreIri {
        &self.store
    }

    /// The query language.
    #[must_use]
    pub const fn language(&self) -> &QueryLanguage {
        &self.language
    }

    /// The query text.
    #[must_use]
    pub const fn text(&self) -> &QueryText {
        &self.text
    }

    /// What a store must support to answer this request.
    #[must_use]
    pub fn required_capabilities(&self) -> RequiredCapabilities {
        RequiredCapabilities::single(self.language.clone())
    }

    /// Confirms `description` is the addressed store and can answer.
    ///
    /// # Errors
    ///
    /// [`FacadeError::StoreMismatch`] or
    /// [`FacadeError::MissingCapabilities`].
    pub fn check_against(&self, description: &StoreDescription) -> Result<(), FacadeError> {
        if description.iri() != &self.store {
            return Err(FacadeError::StoreMismatch {
                requested: Box::new(self.store.clone()),
                actual: Box::new(description.iri().clone()),
            });
        }
        description.require(&self.required_capabilities())
    }

    /// The key a response to this request may be cached under.
    #[must_use]
    pub fn cache_key(&self) -> CacheKey {
        CacheKey::for_request(self)
    }
}
