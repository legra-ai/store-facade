/// The store-facade namespace (proposed; awaiting owner confirmation).
pub const SF: &str = "https://ns.legra.ai/store-facade#";
/// W3C SPARQL 1.1 Service Description.
pub const SD: &str = "http://www.w3.org/ns/sparql-service-description#";
/// W3C entailment regimes.
pub const ENT: &str = "http://www.w3.org/ns/entailment/";
/// RDF.
pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
/// RDF Schema.
pub const RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
/// XML Schema datatypes.
pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

/// The vendored Turtle definition of the store-facade vocabulary.
pub const TURTLE: &str = include_str!("store-facade.ttl");
