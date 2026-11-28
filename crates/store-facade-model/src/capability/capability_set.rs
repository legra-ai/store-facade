use std::collections::BTreeSet;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Capability,
    DescriptionLimit,
    FacadeError,
    MissingCapabilities,
    RequiredCapabilities,
};

/// The capabilities of one store, ordered and without duplicates.
///
/// bounded: at most [`DescriptionLimit::Capabilities`] entries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Vec<Capability>", into = "Vec<Capability>")]
pub struct CapabilitySet(BTreeSet<Capability>);

impl CapabilitySet {
    /// An empty set.
    #[must_use]
    pub const fn new() -> Self {
        Self(BTreeSet::new())
    }

    /// A set of one capability.
    #[must_use]
    pub fn single(capability: Capability) -> Self {
        Self(BTreeSet::from([capability]))
    }

    /// Collects capabilities, rejecting too many distinct ones.
    ///
    /// # Errors
    ///
    /// [`FacadeError::LimitExceeded`] past [`DescriptionLimit::Capabilities`].
    pub fn try_from_iter(
        capabilities: impl IntoIterator<Item = Capability>,
    ) -> Result<Self, FacadeError> {
        let mut set = Self::new();
        for capability in capabilities {
            set.insert(capability)?;
        }
        Ok(set)
    }

    /// Collects capabilities a caller has already bounded below
    /// [`DescriptionLimit::Capabilities`].
    pub(crate) fn within_limit(capabilities: impl IntoIterator<Item = Capability>) -> Self {
        Self(capabilities.into_iter().collect())
    }

    /// Adds a capability; returns whether it was new.
    ///
    /// # Errors
    ///
    /// [`FacadeError::LimitExceeded`] past [`DescriptionLimit::Capabilities`].
    pub fn insert(&mut self, capability: Capability) -> Result<bool, FacadeError> {
        if !self.0.contains(&capability) && self.0.len() >= DescriptionLimit::Capabilities.max() {
            return Err(FacadeError::LimitExceeded {
                limit: DescriptionLimit::Capabilities,
            });
        }
        Ok(self.0.insert(capability))
    }

    /// Whether the set holds `capability`.
    #[must_use]
    pub fn contains(&self, capability: &Capability) -> bool {
        self.0.contains(capability)
    }

    /// The number of capabilities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The capabilities, in order.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.0.iter()
    }

    /// Confirms every required capability is present.
    ///
    /// # Errors
    ///
    /// [`FacadeError::MissingCapabilities`] naming every absent capability.
    pub fn require(&self, required: &RequiredCapabilities) -> Result<(), FacadeError> {
        let missing: BTreeSet<Capability> = required
            .iter()
            .filter(|capability| !self.contains(capability))
            .cloned()
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(FacadeError::MissingCapabilities(MissingCapabilities::new(
                Self(missing),
            )))
        }
    }
}

impl TryFrom<Vec<Capability>> for CapabilitySet {
    type Error = FacadeError;

    fn try_from(capabilities: Vec<Capability>) -> Result<Self, Self::Error> {
        Self::try_from_iter(capabilities)
    }
}

impl From<CapabilitySet> for Vec<Capability> {
    fn from(set: CapabilitySet) -> Self {
        set.0.into_iter().collect()
    }
}

impl<'set> IntoIterator for &'set CapabilitySet {
    type IntoIter = std::collections::btree_set::Iter<'set, Capability>;
    type Item = &'set Capability;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
