//! Description terms, each spelled out in full so constants stay `const`.

/// `rdf:type`.
pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
/// `rdfs:label`: a store's human-readable, language-tagged name, at most one
/// per language tag.
pub const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
/// `sf:Store`: the class of logical stores.
pub const SF_STORE: &str = "https://ns.legra.ai/store-facade#Store";
/// `sf:slug`: a store's [`crate::StoreSlug`], a sub-property of
/// `dcterms:identifier` with a constrained syntax.
pub const SF_SLUG: &str = "https://ns.legra.ai/store-facade#slug";
/// `sd:supportedLanguage`: relates a store to a query or update language.
pub const SD_SUPPORTED_LANGUAGE: &str =
    "http://www.w3.org/ns/sparql-service-description#supportedLanguage";
/// `sd:feature`: relates a store to a feature it supports.
pub const SD_FEATURE: &str = "http://www.w3.org/ns/sparql-service-description#feature";
/// `sd:entailmentRegime`: relates a store to an entailment regime it applies.
pub const SD_ENTAILMENT_REGIME: &str =
    "http://www.w3.org/ns/sparql-service-description#entailmentRegime";
/// `sd:Language`: the class of language capabilities.
pub const SD_LANGUAGE_CLASS: &str = "http://www.w3.org/ns/sparql-service-description#Language";
/// `sd:Feature`: the class of feature capabilities.
pub const SD_FEATURE_CLASS: &str = "http://www.w3.org/ns/sparql-service-description#Feature";
/// `sd:EntailmentRegime`: the class of entailment-regime capabilities.
pub const SD_ENTAILMENT_REGIME_CLASS: &str =
    "http://www.w3.org/ns/sparql-service-description#EntailmentRegime";
