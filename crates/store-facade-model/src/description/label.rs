use core::fmt;

use rdf_structural_model::{
    LanguageTag,
    Literal,
    LiteralFacets,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    DescriptionLimit,
    FacadeError,
};

/// Non-empty label text of at most [`DescriptionLimit::LabelBytes`] bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LabelText(String);

impl LabelText {
    /// Checks and wraps label text.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidLabel`] when empty, or
    /// [`FacadeError::LimitExceeded`] when too long.
    pub fn try_new(text: impl Into<String>) -> Result<Self, FacadeError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(FacadeError::InvalidLabel {
                reason: "label text must not be blank",
            });
        }
        if text.len() > DescriptionLimit::LabelBytes.max() {
            return Err(FacadeError::LimitExceeded {
                limit: DescriptionLimit::LabelBytes,
            });
        }
        Ok(Self(text))
    }

    /// The text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for LabelText {
    type Error = FacadeError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::try_new(text)
    }
}

impl From<LabelText> for String {
    fn from(text: LabelText) -> Self {
        text.0
    }
}

impl fmt::Display for LabelText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One language-tagged store label, written as `rdfs:label`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreLabel {
    language: LanguageTag,
    text: LabelText,
}

impl StoreLabel {
    /// A label in `language`.
    #[must_use]
    pub const fn new(language: LanguageTag, text: LabelText) -> Self {
        Self { language, text }
    }

    /// The label's language tag.
    #[must_use]
    pub const fn language(&self) -> &LanguageTag {
        &self.language
    }

    /// The label's text.
    #[must_use]
    pub const fn text(&self) -> &LabelText {
        &self.text
    }

    /// The label as an RDF language-tagged literal.
    #[must_use]
    pub fn to_literal(&self) -> Literal {
        Literal::language_tagged(self.text.as_str(), self.language.clone())
    }

    /// Reads a label from a language-tagged literal.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidLabel`] when the literal has no language tag or
    /// carries a base direction.
    pub fn try_from_literal(literal: &Literal) -> Result<Self, FacadeError> {
        let LiteralFacets::Language(language) = literal.metadata().facets() else {
            return Err(FacadeError::InvalidLabel {
                reason: "a label is a language-tagged literal without a base direction",
            });
        };
        Ok(Self::new(
            language.clone(),
            LabelText::try_new(literal.lexical())?,
        ))
    }

    pub(super) fn into_parts(self) -> (LanguageTag, LabelText) {
        (self.language, self.text)
    }
}
