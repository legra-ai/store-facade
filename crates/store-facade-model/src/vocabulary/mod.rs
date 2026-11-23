//! The store-facade vocabulary: namespaces, description terms and capability
//! constants.
//!
//! Capabilities reuse W3C SPARQL 1.1 Service Description terms
//! (`sd:supportedLanguage`, `sd:feature`, `sd:entailmentRegime`) and W3C
//! entailment-regime IRIs wherever they exist. Terms without a W3C
//! equivalent live in the store-facade namespace [`SF`]. [`TURTLE`] is the
//! vendored vocabulary that defines them.

mod capabilities;
mod capability_term;
mod namespaces;
mod terms;

pub use capabilities::{
    ALL_CAPABILITIES,
    ENTAILMENT_OWL_RDF_BASED,
    ENTAILMENT_RDFS,
    EXACT_LITERAL_PRESERVATION,
    FULL_TEXT_SEARCH,
    GEOSPARQL,
    GRAPH_STORE_PROTOCOL,
    NAMED_GRAPHS,
    PER_QUERY_TIMEOUT,
    RDF12_TERMS,
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
    TRANSACTIONS,
    UNION_DEFAULT_GRAPH,
};
pub use capability_term::CapabilityTerm;
pub use namespaces::{
    ENT,
    RDF,
    RDFS,
    SD,
    SF,
    TURTLE,
    XSD,
};
pub use terms::{
    RDF_TYPE,
    RDFS_LABEL,
    SD_ENTAILMENT_REGIME,
    SD_ENTAILMENT_REGIME_CLASS,
    SD_FEATURE,
    SD_FEATURE_CLASS,
    SD_LANGUAGE_CLASS,
    SD_SUPPORTED_LANGUAGE,
    SF_SLUG,
    SF_STORE,
};
