use core::fmt;

use serde::{
    Deserialize,
    Serialize,
};

use crate::vocabulary::{
    SD_ENTAILMENT_REGIME,
    SD_ENTAILMENT_REGIME_CLASS,
    SD_FEATURE,
    SD_FEATURE_CLASS,
    SD_LANGUAGE_CLASS,
    SD_SUPPORTED_LANGUAGE,
};

/// What a capability is, which fixes the Service Description property that
/// relates a store to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityRole {
    /// A query or update language (`sd:supportedLanguage`).
    Language,
    /// A feature (`sd:feature`).
    Feature,
    /// An entailment regime (`sd:entailmentRegime`).
    EntailmentRegime,
}

impl CapabilityRole {
    /// Every role.
    pub const ALL: [Self; 3] = [Self::Language, Self::Feature, Self::EntailmentRegime];

    /// The property relating a store to a capability of this role.
    #[must_use]
    pub const fn predicate(self) -> &'static str {
        match self {
            Self::Language => SD_SUPPORTED_LANGUAGE,
            Self::Feature => SD_FEATURE,
            Self::EntailmentRegime => SD_ENTAILMENT_REGIME,
        }
    }

    /// The class of capabilities of this role.
    #[must_use]
    pub const fn class(self) -> &'static str {
        match self {
            Self::Language => SD_LANGUAGE_CLASS,
            Self::Feature => SD_FEATURE_CLASS,
            Self::EntailmentRegime => SD_ENTAILMENT_REGIME_CLASS,
        }
    }

    /// The role whose [`CapabilityRole::predicate`] is `predicate`.
    #[must_use]
    pub fn from_predicate(predicate: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|role| role.predicate() == predicate)
    }
}

impl fmt::Display for CapabilityRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Language => "language",
            Self::Feature => "feature",
            Self::EntailmentRegime => "entailment-regime",
        })
    }
}
