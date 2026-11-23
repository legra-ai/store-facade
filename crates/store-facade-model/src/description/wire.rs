use serde::Deserialize;

use crate::{
    CapabilitySet,
    FacadeError,
    StoreDescription,
    StoreIri,
    StoreLabels,
    StoreSlug,
};

/// The serialized fields, checked by [`StoreDescription::try_new`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DescriptionFields {
    iri: StoreIri,
    slug: StoreSlug,
    labels: StoreLabels,
    capabilities: CapabilitySet,
}

impl TryFrom<DescriptionFields> for StoreDescription {
    type Error = FacadeError;

    fn try_from(fields: DescriptionFields) -> Result<Self, Self::Error> {
        Self::try_new(fields.iri, fields.slug, fields.labels, fields.capabilities)
    }
}
