use crate::{
    Capability,
    CapabilityRole,
};

/// A vocabulary capability: a `const` role and IRI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityTerm {
    role: CapabilityRole,
    iri: &'static str,
}

impl CapabilityTerm {
    pub(super) const fn new(role: CapabilityRole, iri: &'static str) -> Self {
        Self { role, iri }
    }

    /// The capability's role.
    #[must_use]
    pub const fn role(self) -> CapabilityRole {
        self.role
    }

    /// The capability's IRI.
    #[must_use]
    pub const fn iri(self) -> &'static str {
        self.iri
    }

    /// The owned capability.
    #[must_use]
    pub fn capability(self) -> Capability {
        Capability::known(self.role, self.iri)
    }
}

impl From<CapabilityTerm> for Capability {
    fn from(term: CapabilityTerm) -> Self {
        term.capability()
    }
}
