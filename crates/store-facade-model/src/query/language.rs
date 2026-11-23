use core::fmt;

use serde::{
    Deserialize,
    Serialize,
};

use crate::vocabulary::CapabilityTerm;
use crate::{
    Capability,
    CapabilityRole,
    FacadeError,
};

/// The language of a query: a capability in the
/// [`CapabilityRole::Language`] role, such as
/// [`crate::vocabulary::SPARQL11_QUERY`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "Capability", into = "Capability")]
pub struct QueryLanguage(Capability);

impl QueryLanguage {
    /// Checks that `capability` is a language.
    ///
    /// # Errors
    ///
    /// [`FacadeError::CapabilityRoleMismatch`] for any other role.
    pub fn try_new(capability: Capability) -> Result<Self, FacadeError> {
        capability.expect_role(CapabilityRole::Language).map(Self)
    }

    /// The language as a capability.
    #[must_use]
    pub const fn capability(&self) -> &Capability {
        &self.0
    }
}

impl TryFrom<Capability> for QueryLanguage {
    type Error = FacadeError;

    fn try_from(capability: Capability) -> Result<Self, Self::Error> {
        Self::try_new(capability)
    }
}

impl TryFrom<CapabilityTerm> for QueryLanguage {
    type Error = FacadeError;

    fn try_from(term: CapabilityTerm) -> Result<Self, Self::Error> {
        Self::try_new(term.capability())
    }
}

impl From<QueryLanguage> for Capability {
    fn from(language: QueryLanguage) -> Self {
        language.0
    }
}

impl fmt::Display for QueryLanguage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
