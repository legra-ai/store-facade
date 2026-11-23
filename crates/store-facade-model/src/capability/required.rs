use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Capability,
    CapabilitySet,
    FacadeError,
};

/// The capabilities a caller needs from a store.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequiredCapabilities(CapabilitySet);

impl RequiredCapabilities {
    /// Requires exactly `capabilities`.
    #[must_use]
    pub const fn new(capabilities: CapabilitySet) -> Self {
        Self(capabilities)
    }

    /// Requires one capability.
    #[must_use]
    pub fn single(capability: impl Into<Capability>) -> Self {
        Self(CapabilitySet::single(capability.into()))
    }

    /// Requires the given capabilities.
    ///
    /// # Errors
    ///
    /// [`FacadeError::LimitExceeded`] for too many distinct capabilities.
    pub fn try_from_iter(
        capabilities: impl IntoIterator<Item = Capability>,
    ) -> Result<Self, FacadeError> {
        CapabilitySet::try_from_iter(capabilities).map(Self)
    }

    /// Also requires `capability`.
    ///
    /// # Errors
    ///
    /// [`FacadeError::LimitExceeded`] for too many distinct capabilities.
    pub fn with(mut self, capability: impl Into<Capability>) -> Result<Self, FacadeError> {
        self.0.insert(capability.into())?;
        Ok(self)
    }

    /// The required capabilities, in order.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.0.iter()
    }

    /// The required capabilities as a set.
    #[must_use]
    pub const fn as_set(&self) -> &CapabilitySet {
        &self.0
    }
}
