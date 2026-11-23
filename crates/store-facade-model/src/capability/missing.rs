use core::fmt;

use crate::CapabilitySet;

/// The capabilities a store lacks for a requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingCapabilities(CapabilitySet);

impl MissingCapabilities {
    pub(crate) const fn new(missing: CapabilitySet) -> Self {
        Self(missing)
    }

    /// The absent capabilities, never empty.
    #[must_use]
    pub const fn capabilities(&self) -> &CapabilitySet {
        &self.0
    }
}

impl fmt::Display for MissingCapabilities {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the store lacks required capabilities:")?;
        for capability in &self.0 {
            write!(formatter, " <{capability}>")?;
        }
        Ok(())
    }
}

impl core::error::Error for MissingCapabilities {}
