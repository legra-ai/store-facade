//! JSON and Postcard forms.

mod support;

use store_facade_model::vocabulary::SPARQL11_QUERY;
use store_facade_model::{
    QueryLanguage,
    QueryRequest,
    QueryText,
    StoreDescription,
};
use support::{
    sample,
    store_iri,
};

const GOLDEN: &str = include_str!("fixtures/golden/sales.json");

#[test]
fn json_matches_the_golden_file() {
    let json = serde_json::to_string_pretty(&sample()).expect("serializes") + "\n";
    assert_eq!(json, GOLDEN);
    assert_eq!(
        serde_json::from_str::<StoreDescription>(GOLDEN).expect("parses"),
        sample()
    );
}

#[test]
fn postcard_round_trips() {
    let bytes = postcard::to_allocvec(&sample()).expect("serializes");
    assert_eq!(
        postcard::from_bytes::<StoreDescription>(&bytes).expect("parses"),
        sample()
    );
    let request = QueryRequest::new(
        store_iri(),
        QueryLanguage::try_from(SPARQL11_QUERY).expect("language"),
        QueryText::try_new("ASK {}").expect("text"),
    );
    let bytes = postcard::to_allocvec(&request).expect("serializes");
    assert_eq!(
        postcard::from_bytes::<QueryRequest>(&bytes).expect("parses"),
        request
    );
}

#[test]
fn json_deserialization_enforces_invariants() {
    let no_labels = r#"{"iri":"https://example.org/s","slug":"s","labels":[],"capabilities":[]}"#;
    assert!(serde_json::from_str::<StoreDescription>(no_labels).is_err());
    let extra = GOLDEN.replacen('{', "{\"product\": \"engine\",", 1);
    assert!(serde_json::from_str::<StoreDescription>(&extra).is_err());
    let bad_slug = GOLDEN.replacen("\"sales\"", "\"Sales\"", 1);
    assert!(serde_json::from_str::<StoreDescription>(&bad_slug).is_err());
    let duplicate_language = r#"{"iri":"https://example.org/s","slug":"s","labels":[
        {"language":"en","text":"A"},{"language":"EN","text":"B"}],"capabilities":[]}"#;
    assert!(serde_json::from_str::<StoreDescription>(duplicate_language).is_err());
}
