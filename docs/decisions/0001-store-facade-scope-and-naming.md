# 0001: Store-facade scope and naming

Status: Accepted

## Context

Applications query RDF stores through many products, each with its own
endpoint conventions, feature set and deployment model. Code that names a
product, a cluster or a region couples every caller to an operator decision
that should be free to change. Callers need one way to address a logical
store and to learn, before sending a query, whether that store can answer it.

## Decision

store-facade is an open-source Rust facade over query stores, with Python
wheels later.

- **Store IRI.** A caller addresses a logical store only by its Store IRI, an
  absolute RFC 3987 IRI.
- **Store description.** A caller holds a small RDF graph that says of one
  Store IRI its slug, its language-tagged labels and its capabilities, and
  nothing else.
- **Capabilities are IRIs** from a small vocabulary of our own that reuses the
  W3C SPARQL 1.1 Service Description properties `sd:supportedLanguage`,
  `sd:feature` and `sd:entailmentRegime`, and the W3C terms they relate to
  (`sd:SPARQL11Query`, `sd:SPARQL11Update`, `sd:UnionDefaultGraph`, the
  `http://www.w3.org/ns/entailment/` regimes) wherever those exist. Terms
  without a W3C equivalent, such as SPARQL 1.2, the Graph Store Protocol,
  RDF 1.2 terms, exact literal preservation, transactions, named graphs,
  full-text search, GeoSPARQL and per-query timeouts, live in our namespace.
  Later query languages (GraphQL, Cypher, SQL) are capability IRIs too, so a
  query language is a capability in the language role.
- **Hidden technology.** Nothing in a description names the product, cluster,
  topology, region or lifecycle (always-on or per call). That mapping lives in
  an operator-side binding, which is outside this decision's first slice.
- **Public specifications only.** The design derives from W3C specifications.
  No existing proprietary proxy package is consulted or reused.

### Description vocabulary

- `rdf:type sf:Store` types the store.
- `sf:slug`, an `xsd:string` sub-property of `dcterms:identifier`, carries the
  slug. No established property expresses a constrained, DNS-label-safe
  handle, so we mint one and relate it to the closest Dublin Core term.
- `rdfs:label` carries the labels, at most one per language tag. It is the
  most widely understood labelling property; `skos:prefLabel` was considered,
  but adds a SKOS dependency for no gain once the model enforces one label
  per language itself.

The namespace is `https://ns.legra.ai/store-facade#` (prefix `sf`),
confirmed by the owner on 2026-11-23. Its definition is vendored with the model crate.

### Naming ruling

"store-facade" everywhere: the model crate `store-facade-model`, the service
`store-facade`, each backend `store-facade-<backend>`, and the same names for
the Python wheels.

## Consequences

The first slice (S1) delivers `store-facade-model`: the identity types, the
description with streaming RDF read and write, the capability vocabulary with
a census test against its Turtle file, capability requirements, the async
streaming and cancellable query trait, the stable error-code catalogue, and
unused cache hooks. The model has no RDF parser dependency; callers feed it
triples from any parser.

Readers reject statements about a store that a description cannot hold, such
as extra types or unknown predicates, so technology details cannot slip into
a description unnoticed.
