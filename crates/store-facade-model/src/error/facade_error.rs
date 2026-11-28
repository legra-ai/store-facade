use crate::{
    Capability,
    CapabilityRole,
    DescriptionLimit,
    ErrorCode,
    ImplementationName,
    MissingCapabilities,
    SlugRejection,
    StoreIri,
};

/// Every failure the store-facade model reports.
///
/// Each variant maps to exactly one stable [`ErrorCode`] through
/// [`FacadeError::code`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum FacadeError {
    /// A Store IRI is not an absolute RFC 3987 IRI.
    #[error("invalid Store IRI: {reason}")]
    InvalidStoreIri {
        /// The parser's explanation.
        reason: String,
    },
    /// A store slug is malformed.
    #[error("invalid store slug {slug:?}: {reason}")]
    InvalidStoreSlug {
        /// The rejected candidate.
        slug: String,
        /// Why it was rejected.
        reason: SlugRejection,
    },
    /// A capability IRI is not an absolute RFC 3987 IRI.
    #[error("invalid capability IRI: {reason}")]
    InvalidCapability {
        /// The parser's explanation.
        reason: String,
    },
    /// A capability was used in a role it does not have.
    #[error("capability <{capability}> is not a {expected} capability")]
    CapabilityRoleMismatch {
        /// The capability supplied.
        capability: Box<Capability>,
        /// The role the context requires.
        expected: CapabilityRole,
    },
    /// A store label is malformed.
    #[error("invalid store label: {reason}")]
    InvalidLabel {
        /// Why the label was rejected.
        reason: &'static str,
    },
    /// A store description has no label.
    #[error("a store description needs at least one language-tagged label")]
    MissingLabel,
    /// A store description has no slug.
    #[error("a store description needs exactly one slug")]
    MissingSlug,
    /// A bounded collection would exceed its limit.
    #[error("{limit} exceeded")]
    LimitExceeded {
        /// The limit that was hit.
        limit: DescriptionLimit,
    },
    /// A store lacks capabilities a caller requires.
    #[error(transparent)]
    MissingCapabilities(MissingCapabilities),
    /// An RDF statement about a store cannot belong to a store description.
    #[error("invalid store description RDF: {reason}")]
    InvalidDescriptionRdf {
        /// Why the statement was rejected.
        reason: &'static str,
    },
    /// A query request addresses a different store than the one asked.
    #[error("the request addresses <{requested}>, not <{actual}>")]
    StoreMismatch {
        /// The store the request names.
        requested: Box<StoreIri>,
        /// The store that received it.
        actual: Box<StoreIri>,
    },
    /// A query text is empty.
    #[error("a query text must not be empty")]
    EmptyQuery,
    /// An entity tag is malformed.
    #[error("invalid entity tag: {reason}")]
    InvalidEntityTag {
        /// Why the tag was rejected.
        reason: &'static str,
    },
    /// A cache key is not 64 lowercase hexadecimal digits.
    #[error("a cache key must be 64 lowercase hexadecimal digits")]
    InvalidCacheKey,
    /// The caller cancelled a query.
    #[error("the query was cancelled")]
    Cancelled,
    /// A query text is not valid in its language.
    #[error("invalid query: {message}")]
    InvalidQuery {
        /// A technology-neutral explanation.
        message: String,
    },
    /// A query timeout is out of range.
    #[error("invalid query timeout: {reason}")]
    InvalidQueryTimeout {
        /// Why the timeout was rejected.
        reason: &'static str,
    },
    /// A query ran past its timeout.
    #[error("the query exceeded its timeout")]
    Timeout,
    /// An implementation name is malformed.
    #[error("invalid implementation name {name:?}: {reason}")]
    InvalidImplementationName {
        /// The rejected candidate.
        name: String,
        /// Why it was rejected.
        reason: SlugRejection,
    },
    /// A binding names another implementation than the binder it was given.
    #[error("the binding selects implementation {actual}, not {expected}")]
    BindingMismatch {
        /// The binder's implementation.
        expected: Box<ImplementationName>,
        /// The implementation the binding selects.
        actual: Box<ImplementationName>,
    },
    /// A backend failed; the message never names the technology behind it.
    #[error("the store failed: {message}")]
    Backend {
        /// A technology-neutral explanation.
        message: String,
    },
}

impl FacadeError {
    /// Code of [`FacadeError::InvalidStoreIri`].
    pub const CODE_INVALID_STORE_IRI: ErrorCode = ErrorCode::new("invalid-store-iri");
    /// Code of [`FacadeError::InvalidStoreSlug`].
    pub const CODE_INVALID_STORE_SLUG: ErrorCode = ErrorCode::new("invalid-store-slug");
    /// Code of [`FacadeError::InvalidCapability`].
    pub const CODE_INVALID_CAPABILITY: ErrorCode = ErrorCode::new("invalid-capability");
    /// Code of [`FacadeError::CapabilityRoleMismatch`].
    pub const CODE_CAPABILITY_ROLE_MISMATCH: ErrorCode = ErrorCode::new("capability-role-mismatch");
    /// Code of [`FacadeError::InvalidLabel`].
    pub const CODE_INVALID_LABEL: ErrorCode = ErrorCode::new("invalid-label");
    /// Code of [`FacadeError::MissingLabel`].
    pub const CODE_MISSING_LABEL: ErrorCode = ErrorCode::new("missing-label");
    /// Code of [`FacadeError::MissingSlug`].
    pub const CODE_MISSING_SLUG: ErrorCode = ErrorCode::new("missing-slug");
    /// Code of [`FacadeError::LimitExceeded`].
    pub const CODE_LIMIT_EXCEEDED: ErrorCode = ErrorCode::new("limit-exceeded");
    /// Code of [`FacadeError::MissingCapabilities`].
    pub const CODE_MISSING_CAPABILITIES: ErrorCode = ErrorCode::new("missing-capabilities");
    /// Code of [`FacadeError::InvalidDescriptionRdf`].
    pub const CODE_INVALID_DESCRIPTION_RDF: ErrorCode = ErrorCode::new("invalid-description-rdf");
    /// Code of [`FacadeError::StoreMismatch`].
    pub const CODE_STORE_MISMATCH: ErrorCode = ErrorCode::new("store-mismatch");
    /// Code of [`FacadeError::EmptyQuery`].
    pub const CODE_EMPTY_QUERY: ErrorCode = ErrorCode::new("empty-query");
    /// Code of [`FacadeError::InvalidEntityTag`].
    pub const CODE_INVALID_ENTITY_TAG: ErrorCode = ErrorCode::new("invalid-entity-tag");
    /// Code of [`FacadeError::InvalidCacheKey`].
    pub const CODE_INVALID_CACHE_KEY: ErrorCode = ErrorCode::new("invalid-cache-key");
    /// Code of [`FacadeError::Cancelled`].
    pub const CODE_CANCELLED: ErrorCode = ErrorCode::new("cancelled");
    /// Code of [`FacadeError::Backend`].
    pub const CODE_BACKEND: ErrorCode = ErrorCode::new("backend");
    /// Code of [`FacadeError::InvalidQuery`].
    pub const CODE_INVALID_QUERY: ErrorCode = ErrorCode::new("invalid-query");
    /// Code of [`FacadeError::InvalidQueryTimeout`].
    pub const CODE_INVALID_QUERY_TIMEOUT: ErrorCode = ErrorCode::new("invalid-query-timeout");
    /// Code of [`FacadeError::Timeout`].
    pub const CODE_TIMEOUT: ErrorCode = ErrorCode::new("timeout");
    /// Code of [`FacadeError::InvalidImplementationName`].
    pub const CODE_INVALID_IMPLEMENTATION_NAME: ErrorCode =
        ErrorCode::new("invalid-implementation-name");
    /// Code of [`FacadeError::BindingMismatch`].
    pub const CODE_BINDING_MISMATCH: ErrorCode = ErrorCode::new("binding-mismatch");

    /// The complete catalogue, in declaration order.
    pub const CODES: [ErrorCode; 21] = [
        Self::CODE_INVALID_STORE_IRI,
        Self::CODE_INVALID_STORE_SLUG,
        Self::CODE_INVALID_CAPABILITY,
        Self::CODE_CAPABILITY_ROLE_MISMATCH,
        Self::CODE_INVALID_LABEL,
        Self::CODE_MISSING_LABEL,
        Self::CODE_MISSING_SLUG,
        Self::CODE_LIMIT_EXCEEDED,
        Self::CODE_MISSING_CAPABILITIES,
        Self::CODE_INVALID_DESCRIPTION_RDF,
        Self::CODE_STORE_MISMATCH,
        Self::CODE_EMPTY_QUERY,
        Self::CODE_INVALID_ENTITY_TAG,
        Self::CODE_INVALID_CACHE_KEY,
        Self::CODE_CANCELLED,
        Self::CODE_BACKEND,
        Self::CODE_INVALID_QUERY,
        Self::CODE_INVALID_QUERY_TIMEOUT,
        Self::CODE_TIMEOUT,
        Self::CODE_INVALID_IMPLEMENTATION_NAME,
        Self::CODE_BINDING_MISMATCH,
    ];

    /// This error's stable code.
    #[must_use]
    pub const fn code(&self) -> ErrorCode {
        match self {
            Self::InvalidStoreIri { .. } => Self::CODE_INVALID_STORE_IRI,
            Self::InvalidStoreSlug { .. } => Self::CODE_INVALID_STORE_SLUG,
            Self::InvalidCapability { .. } => Self::CODE_INVALID_CAPABILITY,
            Self::CapabilityRoleMismatch { .. } => Self::CODE_CAPABILITY_ROLE_MISMATCH,
            Self::InvalidLabel { .. } => Self::CODE_INVALID_LABEL,
            Self::MissingLabel => Self::CODE_MISSING_LABEL,
            Self::MissingSlug => Self::CODE_MISSING_SLUG,
            Self::LimitExceeded { .. } => Self::CODE_LIMIT_EXCEEDED,
            Self::MissingCapabilities(_) => Self::CODE_MISSING_CAPABILITIES,
            Self::InvalidDescriptionRdf { .. } => Self::CODE_INVALID_DESCRIPTION_RDF,
            Self::StoreMismatch { .. } => Self::CODE_STORE_MISMATCH,
            Self::EmptyQuery => Self::CODE_EMPTY_QUERY,
            Self::InvalidEntityTag { .. } => Self::CODE_INVALID_ENTITY_TAG,
            Self::InvalidCacheKey => Self::CODE_INVALID_CACHE_KEY,
            Self::Cancelled => Self::CODE_CANCELLED,
            Self::Backend { .. } => Self::CODE_BACKEND,
            Self::InvalidQuery { .. } => Self::CODE_INVALID_QUERY,
            Self::InvalidQueryTimeout { .. } => Self::CODE_INVALID_QUERY_TIMEOUT,
            Self::Timeout => Self::CODE_TIMEOUT,
            Self::InvalidImplementationName { .. } => Self::CODE_INVALID_IMPLEMENTATION_NAME,
            Self::BindingMismatch { .. } => Self::CODE_BINDING_MISMATCH,
        }
    }

    /// An invalid query with a technology-neutral message.
    #[must_use]
    pub fn invalid_query(message: impl Into<String>) -> Self {
        Self::InvalidQuery {
            message: message.into(),
        }
    }

    /// A backend failure with a technology-neutral message.
    #[must_use]
    pub fn backend(message: impl Into<String>) -> Self {
        Self::Backend {
            message: message.into(),
        }
    }
}
