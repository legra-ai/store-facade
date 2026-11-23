//! Property tests: any valid description round-trips through RDF, JSON and
//! Postcard unchanged.

mod support;

use proptest::prelude::*;
use proptest::sample::subsequence;
use rdf_structural_model::Triple;
use store_facade_model::vocabulary::ALL_CAPABILITIES;
use store_facade_model::{
    Capability,
    CapabilityRole,
    StoreDescription,
    StoreIri,
    StoreSlug,
};

fn description() -> impl Strategy<Value = StoreDescription> {
    let languages = ["en", "nl", "de", "fr-CA", "zh-Hant", "x-private"];
    (
        "[a-z][a-z0-9]{0,8}",
        "[a-z]([a-z0-9]|-[a-z0-9]){0,20}",
        subsequence(languages.to_vec(), 1..=languages.len()),
        prop::collection::vec("[^\\p{C}]{0,20}[a-zA-Z]", 6),
        subsequence(ALL_CAPABILITIES.to_vec(), 0..=ALL_CAPABILITIES.len()),
        prop::collection::btree_set("[a-z]{1,8}", 0..4),
    )
        .prop_map(|(host, slug, languages, texts, catalogue, custom)| {
            let iri = StoreIri::try_new(format!("https://{host}.example/store")).expect("valid");
            let builder = StoreDescription::builder(iri, StoreSlug::try_new(slug).expect("valid"));
            let builder = languages
                .into_iter()
                .zip(texts)
                .fold(builder, |builder, (language, text)| {
                    builder.label(support::tag(language), text)
                });
            let custom = custom.into_iter().map(|name| {
                Capability::try_new(CapabilityRole::Feature, format!("urn:feature:{name}"))
                    .expect("valid")
            });
            builder
                .capabilities(catalogue.into_iter().map(Capability::from).chain(custom))
                .build()
                .expect("valid")
        })
}

proptest! {
    #[test]
    fn rdf_round_trip(description in description()) {
        let triples: Vec<Triple> = description.triples().collect();
        prop_assert_eq!(
            triples.len(),
            2 + description.labels().len() + description.capabilities().len()
        );
        let read = StoreDescription::from_triples(description.iri().clone(), triples.iter().rev())
            .expect("reads");
        prop_assert_eq!(read, description);
    }

    #[test]
    fn json_round_trip(description in description()) {
        let json = serde_json::to_string(&description).expect("serializes");
        prop_assert_eq!(serde_json::from_str::<StoreDescription>(&json).expect("parses"), description);
    }

    #[test]
    fn postcard_round_trip(description in description()) {
        let bytes = postcard::to_allocvec(&description).expect("serializes");
        prop_assert_eq!(postcard::from_bytes::<StoreDescription>(&bytes).expect("parses"), description);
    }
}
