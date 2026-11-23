use core::fmt;

use serde::{
    Deserialize,
    Serialize,
};

use crate::FacadeError;

/// The opaque text of an entity tag (RFC 9110 `etagc` characters only).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EntityTag(String);

impl EntityTag {
    /// The longest accepted tag, in bytes.
    pub const MAX_LEN: usize = 256;

    /// Checks and wraps an entity tag.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidEntityTag`] when empty, too long, or holding a
    /// character outside `etagc`.
    pub fn try_new(tag: impl Into<String>) -> Result<Self, FacadeError> {
        let tag = tag.into();
        if tag.is_empty() {
            return Err(FacadeError::InvalidEntityTag {
                reason: "it is empty",
            });
        }
        if tag.len() > Self::MAX_LEN {
            return Err(FacadeError::InvalidEntityTag {
                reason: "it exceeds 256 bytes",
            });
        }
        if !tag
            .bytes()
            .all(|byte| byte == 0x21 || (0x23..=0x7e).contains(&byte))
        {
            return Err(FacadeError::InvalidEntityTag {
                reason: "only visible ASCII other than '\"' is allowed",
            });
        }
        Ok(Self(tag))
    }

    /// The tag text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EntityTag {
    type Error = FacadeError;

    fn try_from(tag: String) -> Result<Self, Self::Error> {
        Self::try_new(tag)
    }
}

impl From<EntityTag> for String {
    fn from(tag: EntityTag) -> Self {
        tag.0
    }
}

impl fmt::Display for EntityTag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
