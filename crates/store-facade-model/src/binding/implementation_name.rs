use core::fmt;
use core::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    FacadeError,
    StoreSlug,
};

/// The operator-side name of a store implementation, such as `oxigraph`.
///
/// It follows the [`StoreSlug`] rules. Only bindings and binders carry it; a
/// [`crate::StoreDescription`] never does.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ImplementationName(String);

impl ImplementationName {
    /// Checks and wraps an implementation name.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidImplementationName`] naming the violated rule.
    pub fn try_new(name: impl Into<String>) -> Result<Self, FacadeError> {
        let name = name.into();
        match StoreSlug::rejection(&name) {
            None => Ok(Self(name)),
            Some(reason) => Err(FacadeError::InvalidImplementationName { name, reason }),
        }
    }

    /// The name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ImplementationName {
    type Error = FacadeError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::try_new(name)
    }
}

impl From<ImplementationName> for String {
    fn from(name: ImplementationName) -> Self {
        name.0
    }
}

impl FromStr for ImplementationName {
    type Err = FacadeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::try_new(text)
    }
}

impl fmt::Display for ImplementationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
