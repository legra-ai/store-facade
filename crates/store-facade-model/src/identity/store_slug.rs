use core::fmt;
use core::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    FacadeError,
    SlugRejection,
};

/// A short, human-typable handle for a store, such as `sales-ledger`.
///
/// A slug is 1 to 63 bytes of `a-z`, `0-9` and `-`, starts with a letter,
/// does not end with `-` and contains no `--`. It is a DNS-label-safe name,
/// never an address: callers resolve stores by [`crate::StoreIri`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct StoreSlug(String);

impl StoreSlug {
    /// The longest accepted slug, in bytes.
    pub const MAX_LEN: usize = 63;

    /// Checks and wraps a slug.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidStoreSlug`] naming the violated rule.
    pub fn try_new(slug: impl Into<String>) -> Result<Self, FacadeError> {
        let slug = slug.into();
        match Self::rejection(&slug) {
            None => Ok(Self(slug)),
            Some(reason) => Err(FacadeError::InvalidStoreSlug { slug, reason }),
        }
    }

    /// The first slug rule `slug` violates; shared with
    /// [`crate::ImplementationName`].
    pub(crate) fn rejection(slug: &str) -> Option<SlugRejection> {
        let bytes = slug.as_bytes();
        let Some(first) = bytes.first() else {
            return Some(SlugRejection::Empty);
        };
        if bytes.len() > Self::MAX_LEN {
            return Some(SlugRejection::TooLong);
        }
        if !first.is_ascii_lowercase() {
            return Some(SlugRejection::InvalidStart);
        }
        if !bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        {
            return Some(SlugRejection::InvalidCharacter);
        }
        if slug.ends_with('-') || slug.contains("--") {
            return Some(SlugRejection::InvalidHyphen);
        }
        None
    }

    /// The slug text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for StoreSlug {
    type Error = FacadeError;

    fn try_from(slug: String) -> Result<Self, Self::Error> {
        Self::try_new(slug)
    }
}

impl From<StoreSlug> for String {
    fn from(slug: StoreSlug) -> Self {
        slug.0
    }
}

impl FromStr for StoreSlug {
    type Err = FacadeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::try_new(text)
    }
}

impl fmt::Display for StoreSlug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
