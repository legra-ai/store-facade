use std::collections::BTreeMap;

use rdf_structural_model::LanguageTag;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    DescriptionLimit,
    FacadeError,
    LabelText,
    StoreLabel,
};

/// A store's labels: at most one per language tag.
///
/// bounded: at most [`DescriptionLimit::Labels`] entries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Vec<StoreLabel>", into = "Vec<StoreLabel>")]
pub struct StoreLabels(BTreeMap<LanguageTag, LabelText>);

impl StoreLabels {
    /// No labels.
    #[must_use]
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Adds a label. Restating an identical label is accepted.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidLabel`] for a second, different label in the
    /// same language, or [`FacadeError::LimitExceeded`] past
    /// [`DescriptionLimit::Labels`].
    pub fn insert(&mut self, label: StoreLabel) -> Result<(), FacadeError> {
        let (language, text) = label.into_parts();
        match self.0.get(&language) {
            Some(existing) if *existing == text => Ok(()),
            Some(_) => Err(FacadeError::InvalidLabel {
                reason: "a store has at most one label per language tag",
            }),
            None if self.0.len() >= DescriptionLimit::Labels.max() => {
                Err(FacadeError::LimitExceeded {
                    limit: DescriptionLimit::Labels,
                })
            }
            None => {
                self.0.insert(language, text);
                Ok(())
            }
        }
    }

    /// The label in `language`, matched case-insensitively.
    #[must_use]
    pub fn get(&self, language: &LanguageTag) -> Option<&LabelText> {
        self.0.get(language)
    }

    /// The number of labels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether there are no labels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The labels, ordered by language tag.
    pub fn iter(&self) -> impl Iterator<Item = StoreLabel> + '_ {
        self.0
            .iter()
            .map(|(language, text)| StoreLabel::new(language.clone(), text.clone()))
    }
}

impl TryFrom<Vec<StoreLabel>> for StoreLabels {
    type Error = FacadeError;

    fn try_from(labels: Vec<StoreLabel>) -> Result<Self, Self::Error> {
        let mut collected = Self::new();
        for label in labels {
            collected.insert(label)?;
        }
        Ok(collected)
    }
}

impl From<StoreLabels> for Vec<StoreLabel> {
    fn from(labels: StoreLabels) -> Self {
        labels
            .0
            .into_iter()
            .map(|(language, text)| StoreLabel::new(language, text))
            .collect()
    }
}
