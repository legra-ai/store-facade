use serde::{
    Deserialize,
    Serialize,
};

use crate::vocabulary::PER_QUERY_TIMEOUT;
use crate::{
    CacheKey,
    Capability,
    CapabilitySet,
    DescriptionLimit,
    FacadeError,
    QueryLanguage,
    QueryText,
    QueryTimeout,
    RequiredCapabilities,
    StoreDescription,
    StoreIri,
};

/// A language-tagged query addressed to one store.
///
/// Besides its language, a request may require further capabilities
/// ([`QueryRequest::requiring`]) and carry a [`QueryTimeout`], which requires
/// [`PER_QUERY_TIMEOUT`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryRequest {
    store: StoreIri,
    language: QueryLanguage,
    text: QueryText,
    // bounded: at most `QueryRequest::MAX_REQUIRED_FEATURES` capabilities.
    #[serde(default)]
    requires: CapabilitySet,
    #[serde(default)]
    timeout: Option<QueryTimeout>,
}

impl QueryRequest {
    /// The most capabilities [`QueryRequest::requiring`] accepts.
    pub const MAX_REQUIRED_FEATURES: usize = 64;

    /// A query for `store` in `language`.
    #[must_use]
    pub const fn new(store: StoreIri, language: QueryLanguage, text: QueryText) -> Self {
        Self {
            store,
            language,
            text,
            requires: CapabilitySet::new(),
            timeout: None,
        }
    }

    /// Also requires `capability` of the answering store, such as
    /// [`crate::vocabulary::EXACT_LITERAL_PRESERVATION`].
    ///
    /// # Errors
    ///
    /// [`FacadeError::LimitExceeded`] past
    /// [`QueryRequest::MAX_REQUIRED_FEATURES`].
    pub fn requiring(mut self, capability: impl Into<Capability>) -> Result<Self, FacadeError> {
        let capability = capability.into();
        if !self.requires.contains(&capability)
            && self.requires.len() >= Self::MAX_REQUIRED_FEATURES
        {
            return Err(FacadeError::LimitExceeded {
                limit: DescriptionLimit::Capabilities,
            });
        }
        self.requires.insert(capability)?;
        Ok(self)
    }

    /// Bounds this query's duration, which requires [`PER_QUERY_TIMEOUT`].
    #[must_use]
    pub const fn with_timeout(mut self, timeout: QueryTimeout) -> Self {
        self.timeout = Some(timeout);
        self
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

    /// The caller's timeout, if any.
    #[must_use]
    pub const fn timeout(&self) -> Option<QueryTimeout> {
        self.timeout
    }

    /// What a store must support to answer this request: its language, every
    /// capability it requires and, with a timeout, [`PER_QUERY_TIMEOUT`].
    #[must_use]
    pub fn required_capabilities(&self) -> RequiredCapabilities {
        let timeout = self.timeout.map(|_| PER_QUERY_TIMEOUT.capability());
        // bounded: the language, at most `MAX_REQUIRED_FEATURES` capabilities
        // and the timeout stay far below `DescriptionLimit::Capabilities`.
        let all = core::iter::once(self.language.capability().clone())
            .chain(self.requires.iter().cloned())
            .chain(timeout);
        RequiredCapabilities::new(CapabilitySet::within_limit(all))
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
