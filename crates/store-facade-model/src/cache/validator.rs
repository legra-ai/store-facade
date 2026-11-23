use core::fmt;

use serde::{
    Deserialize,
    Serialize,
};

use crate::EntityTag;

/// A store-issued validator for a cached response, compared as RFC 9110
/// entity tags.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum Validator {
    /// The response is byte-for-byte identical while the tag is unchanged.
    Strong(EntityTag),
    /// The response is semantically equivalent while the tag is unchanged.
    Weak(EntityTag),
}

impl Validator {
    /// The entity tag.
    #[must_use]
    pub const fn tag(&self) -> &EntityTag {
        match self {
            Self::Strong(tag) | Self::Weak(tag) => tag,
        }
    }

    /// Whether this is a weak validator.
    #[must_use]
    pub const fn is_weak(&self) -> bool {
        matches!(self, Self::Weak(_))
    }

    /// Strong comparison: both strong and the tags equal.
    #[must_use]
    pub fn matches_strongly(&self, other: &Self) -> bool {
        !self.is_weak() && !other.is_weak() && self.tag() == other.tag()
    }

    /// Weak comparison: the tags equal, whatever their strength.
    #[must_use]
    pub fn matches_weakly(&self, other: &Self) -> bool {
        self.tag() == other.tag()
    }
}

impl fmt::Display for Validator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strong(tag) => write!(formatter, "\"{tag}\""),
            Self::Weak(tag) => write!(formatter, "W/\"{tag}\""),
        }
    }
}
