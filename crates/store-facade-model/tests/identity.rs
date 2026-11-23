//! Store identity: IRIs and slugs.

use proptest::prelude::*;
use store_facade_model::{
    FacadeError,
    SlugRejection,
    StoreIri,
    StoreSlug,
};

fn rejection(slug: &str) -> SlugRejection {
    match StoreSlug::try_new(slug) {
        Err(FacadeError::InvalidStoreSlug { reason, .. }) => reason,
        other => panic!("expected a slug rejection for {slug:?}, got {other:?}"),
    }
}

#[test]
fn accepts_well_formed_slugs() {
    for slug in ["a", "sales", "sales-ledger-2", "x9", &"a".repeat(63)] {
        assert_eq!(StoreSlug::try_new(slug).expect("valid").as_str(), slug);
    }
}

#[test]
fn names_the_violated_slug_rule() {
    assert_eq!(rejection(""), SlugRejection::Empty);
    assert_eq!(rejection(&"a".repeat(64)), SlugRejection::TooLong);
    assert_eq!(rejection("9lives"), SlugRejection::InvalidStart);
    assert_eq!(rejection("-sales"), SlugRejection::InvalidStart);
    assert_eq!(rejection("Sales"), SlugRejection::InvalidStart);
    assert_eq!(rejection("sales_ledger"), SlugRejection::InvalidCharacter);
    assert_eq!(rejection("salés"), SlugRejection::InvalidCharacter);
    assert_eq!(rejection("sales-"), SlugRejection::InvalidHyphen);
    assert_eq!(rejection("sales--ledger"), SlugRejection::InvalidHyphen);
}

#[test]
fn slug_serde_rejects_invalid_text() {
    assert!(serde_json::from_str::<StoreSlug>("\"Bad Slug\"").is_err());
    let slug: StoreSlug = serde_json::from_str("\"sales\"").expect("valid");
    assert_eq!(
        serde_json::to_string(&slug).expect("serializes"),
        "\"sales\""
    );
}

#[test]
fn store_iris_must_be_absolute() {
    let iri = StoreIri::try_new("https://example.org/store/1").expect("valid");
    assert_eq!(iri.to_string(), "https://example.org/store/1");
    assert_eq!(
        "urn:store:1".parse::<StoreIri>().expect("valid").as_str(),
        "urn:store:1"
    );
    for invalid in ["", "relative/path", "https://exa mple.org/"] {
        let error = StoreIri::try_new(invalid).expect_err("invalid");
        assert_eq!(error.code(), FacadeError::CODE_INVALID_STORE_IRI);
    }
    assert!(serde_json::from_str::<StoreIri>("\"not an iri\"").is_err());
}

proptest! {
    #[test]
    fn generated_slugs_round_trip(slug in "[a-z]([a-z0-9]|-[a-z0-9]){0,30}") {
        let parsed = StoreSlug::try_new(slug.clone()).expect("generated slugs are valid");
        prop_assert_eq!(parsed.as_str(), slug.as_str());
        let json = serde_json::to_string(&parsed).expect("serializes");
        prop_assert_eq!(serde_json::from_str::<StoreSlug>(&json).expect("parses"), parsed);
    }

    #[test]
    fn arbitrary_text_is_accepted_only_when_well_formed(text in "\\PC{0,70}") {
        let well_formed = !text.is_empty()
            && text.len() <= StoreSlug::MAX_LEN
            && text.starts_with(|c: char| c.is_ascii_lowercase())
            && text.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && !text.ends_with('-')
            && !text.contains("--");
        prop_assert_eq!(StoreSlug::try_new(text).is_ok(), well_formed);
    }
}
