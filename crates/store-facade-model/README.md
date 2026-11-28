# store-facade-model

The model of [store-facade](https://github.com/legra-ai/store-facade): how a
caller names a logical store, what it may know about one, and the contract
for querying it.

- [`StoreIri`] is the only address of a store; [`StoreSlug`] is its short,
  validated handle.
- [`StoreDescription`] holds a store's IRI, slug, language-tagged labels and
  [`CapabilitySet`]. It reads from and writes to RDF triples one at a time
  ([`StoreDescription::triples`], [`StoreDescriptionReader`]); no parser is
  required. A description never names the product, cluster, topology, region
  or lifecycle behind the store.
- A [`Capability`] is an IRI in a role (language, feature or entailment
  regime), written with the W3C SPARQL Service Description properties
  `sd:supportedLanguage`, `sd:feature` and `sd:entailmentRegime`. The
  [`vocabulary`] module holds the catalogue and the vendored Turtle file.
- [`RequiredCapabilities`] and [`CapabilitySet::require`] reject a store that
  lacks what a caller needs, naming every missing capability.
- [`QueryStore`] is the async, streaming, cancellable query contract;
  [`QueryRequest`] carries a [`QueryLanguage`], itself a capability, and may
  require further capabilities ([`QueryRequest::requiring`]) or carry a
  [`QueryTimeout`], which requires `sf:PerQueryTimeout`.
- [`StoreBinding`] is the operator side: it selects the
  [`ImplementationName`] and configuration that serve a Store IRI. A
  [`Binder`] opens the store and derives the caller-facing description, which
  never names the implementation.
- [`FacadeError`] has a stable code catalogue ([`FacadeError::CODES`]).
- [`CacheKey`] and [`Validator`] are cache hooks for later slices.

```rust
use rdf_structural_model::LanguageTag;
use store_facade_model::vocabulary::{NAMED_GRAPHS, SPARQL11_QUERY};
use store_facade_model::{RequiredCapabilities, StoreDescription, StoreIri, StoreSlug};

let description = StoreDescription::builder(
    StoreIri::try_new("https://example.org/store/sales")?,
    StoreSlug::try_new("sales")?,
)
.label(LanguageTag::try_new("en".to_owned()).expect("valid tag"), "Sales")
.capability(SPARQL11_QUERY)
.capability(NAMED_GRAPHS)
.build()?;

description.require(&RequiredCapabilities::single(SPARQL11_QUERY))?;
let read_back = StoreDescription::from_triples(
    description.iri().clone(),
    &description.triples().collect::<Vec<_>>(),
)?;
assert_eq!(read_back, description);
# Ok::<(), store_facade_model::FacadeError>(())
```

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.
