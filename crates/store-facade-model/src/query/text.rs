use core::fmt;

use serde::{
    Deserialize,
    Serialize,
};

use crate::FacadeError;

/// The non-empty text of a query, in its [`crate::QueryLanguage`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct QueryText(String);

impl QueryText {
    /// Checks and wraps a query text.
    ///
    /// # Errors
    ///
    /// [`FacadeError::EmptyQuery`] when `text` is blank.
    pub fn try_new(text: impl Into<String>) -> Result<Self, FacadeError> {
        let text = text.into();
        if text.trim().is_empty() {
            Err(FacadeError::EmptyQuery)
        } else {
            Ok(Self(text))
        }
    }

    /// The text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for QueryText {
    type Error = FacadeError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::try_new(text)
    }
}

impl From<QueryText> for String {
    fn from(text: QueryText) -> Self {
        text.0
    }
}

impl fmt::Display for QueryText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
