use core::fmt;

use iri_rfc3987::Iri;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    CapabilityRole,
    FacadeError,
};

/// A capability: an IRI in a [`CapabilityRole`].
///
/// The catalogue lives in [`crate::vocabulary`]; any other absolute IRI is a
/// valid capability too, so new languages and features need no code change.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capability {
    role: CapabilityRole,
    iri: Iri<String>,
}

impl Capability {
    /// Wraps a checked IRI.
    #[must_use]
    pub const fn new(role: CapabilityRole, iri: Iri<String>) -> Self {
        Self { role, iri }
    }

    /// Checks and wraps a capability IRI.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidCapability`] when `iri` is not an absolute IRI.
    pub fn try_new(role: CapabilityRole, iri: impl Into<String>) -> Result<Self, FacadeError> {
        Iri::parse_owned(iri.into())
            .map(|iri| Self::new(role, iri))
            .map_err(|(error, _)| FacadeError::InvalidCapability {
                reason: error.to_string(),
            })
    }

    /// Wraps a vocabulary constant, which is a valid IRI by construction.
    pub(crate) fn known(role: CapabilityRole, iri: &'static str) -> Self {
        Self::new(role, Iri::known(iri))
    }

    /// The capability's role.
    #[must_use]
    pub const fn role(&self) -> CapabilityRole {
        self.role
    }

    /// The capability's IRI.
    #[must_use]
    pub const fn iri(&self) -> &Iri<String> {
        &self.iri
    }

    /// The capability's IRI text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    /// Confirms this capability has `expected` role.
    ///
    /// # Errors
    ///
    /// [`FacadeError::CapabilityRoleMismatch`] for any other role.
    pub fn expect_role(self, expected: CapabilityRole) -> Result<Self, FacadeError> {
        if self.role == expected {
            Ok(self)
        } else {
            Err(FacadeError::CapabilityRoleMismatch {
                capability: Box::new(self),
                expected,
            })
        }
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
