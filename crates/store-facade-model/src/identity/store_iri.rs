use core::fmt;
use core::str::FromStr;

use iri_rfc3987::Iri;
use serde::{
    Deserialize,
    Serialize,
};

use crate::FacadeError;

/// The only address of a logical store: an absolute RFC 3987 IRI.
///
/// A Store IRI says nothing about the product, cluster, topology, region or
/// lifecycle behind the store; an operator-side binding resolves it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StoreIri(Iri<String>);

impl StoreIri {
    /// Checks and wraps a Store IRI.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidStoreIri`] when `iri` is not an absolute IRI.
    pub fn try_new(iri: impl Into<String>) -> Result<Self, FacadeError> {
        Iri::parse_owned(iri.into())
            .map(Self)
            .map_err(|(error, _)| FacadeError::InvalidStoreIri {
                reason: error.to_string(),
            })
    }

    /// Wraps an already checked IRI.
    #[must_use]
    pub const fn from_iri(iri: Iri<String>) -> Self {
        Self(iri)
    }

    /// The IRI text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// The checked IRI.
    #[must_use]
    pub const fn as_iri(&self) -> &Iri<String> {
        &self.0
    }

    /// Consumes the Store IRI, returning the checked IRI.
    #[must_use]
    pub fn into_iri(self) -> Iri<String> {
        self.0
    }
}

impl FromStr for StoreIri {
    type Err = FacadeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::try_new(text)
    }
}

impl fmt::Display for StoreIri {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
