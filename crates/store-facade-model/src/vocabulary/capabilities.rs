//! The capability catalogue.

use super::CapabilityTerm;
use crate::CapabilityRole::{
    EntailmentRegime,
    Feature,
    Language,
};

/// `sd:SPARQL11Query`: SPARQL 1.1 Query.
pub const SPARQL11_QUERY: CapabilityTerm = CapabilityTerm::new(
    Language,
    "http://www.w3.org/ns/sparql-service-description#SPARQL11Query",
);
/// `sd:SPARQL11Update`: SPARQL 1.1 Update.
pub const SPARQL11_UPDATE: CapabilityTerm = CapabilityTerm::new(
    Language,
    "http://www.w3.org/ns/sparql-service-description#SPARQL11Update",
);
/// `sf:SPARQL12Query`: SPARQL 1.2 Query, which Service Description predates.
pub const SPARQL12_QUERY: CapabilityTerm =
    CapabilityTerm::new(Language, "https://ns.legra.ai/store-facade#SPARQL12Query");
/// `sf:SPARQL12Update`: SPARQL 1.2 Update.
pub const SPARQL12_UPDATE: CapabilityTerm =
    CapabilityTerm::new(Language, "https://ns.legra.ai/store-facade#SPARQL12Update");
/// `sf:GraphStoreProtocol`: the SPARQL 1.1 Graph Store HTTP Protocol.
pub const GRAPH_STORE_PROTOCOL: CapabilityTerm = CapabilityTerm::new(
    Feature,
    "https://ns.legra.ai/store-facade#GraphStoreProtocol",
);
/// `sf:RDF12Terms`: RDF 1.2 triple terms and directional literals.
pub const RDF12_TERMS: CapabilityTerm =
    CapabilityTerm::new(Feature, "https://ns.legra.ai/store-facade#RDF12Terms");
/// `sf:ExactLiteralPreservation`: literals return with their exact lexical
/// form, never a canonicalised one.
pub const EXACT_LITERAL_PRESERVATION: CapabilityTerm = CapabilityTerm::new(
    Feature,
    "https://ns.legra.ai/store-facade#ExactLiteralPreservation",
);
/// `sf:Transactions`: atomic multi-operation transactions.
pub const TRANSACTIONS: CapabilityTerm =
    CapabilityTerm::new(Feature, "https://ns.legra.ai/store-facade#Transactions");
/// `sf:NamedGraphs`: named graphs in the dataset.
pub const NAMED_GRAPHS: CapabilityTerm =
    CapabilityTerm::new(Feature, "https://ns.legra.ai/store-facade#NamedGraphs");
/// `sd:UnionDefaultGraph`: the default graph is the union of named graphs.
pub const UNION_DEFAULT_GRAPH: CapabilityTerm = CapabilityTerm::new(
    Feature,
    "http://www.w3.org/ns/sparql-service-description#UnionDefaultGraph",
);
/// `sf:FullTextSearch`: full-text search over literals.
pub const FULL_TEXT_SEARCH: CapabilityTerm =
    CapabilityTerm::new(Feature, "https://ns.legra.ai/store-facade#FullTextSearch");
/// `sf:GeoSPARQL`: OGC `GeoSPARQL` functions and geometry literals.
pub const GEOSPARQL: CapabilityTerm =
    CapabilityTerm::new(Feature, "https://ns.legra.ai/store-facade#GeoSPARQL");
/// `sf:PerQueryTimeout`: the caller may bound each query's duration.
pub const PER_QUERY_TIMEOUT: CapabilityTerm =
    CapabilityTerm::new(Feature, "https://ns.legra.ai/store-facade#PerQueryTimeout");
/// `ent:RDFS`: RDFS entailment.
pub const ENTAILMENT_RDFS: CapabilityTerm =
    CapabilityTerm::new(EntailmentRegime, "http://www.w3.org/ns/entailment/RDFS");
/// `ent:OWL-RDF-Based`: OWL 2 RDF-Based Semantics entailment.
pub const ENTAILMENT_OWL_RDF_BASED: CapabilityTerm = CapabilityTerm::new(
    EntailmentRegime,
    "http://www.w3.org/ns/entailment/OWL-RDF-Based",
);

/// Every capability in the catalogue.
pub const ALL_CAPABILITIES: [CapabilityTerm; 15] = [
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
    GRAPH_STORE_PROTOCOL,
    RDF12_TERMS,
    EXACT_LITERAL_PRESERVATION,
    TRANSACTIONS,
    NAMED_GRAPHS,
    UNION_DEFAULT_GRAPH,
    FULL_TEXT_SEARCH,
    GEOSPARQL,
    PER_QUERY_TIMEOUT,
    ENTAILMENT_RDFS,
    ENTAILMENT_OWL_RDF_BASED,
];
