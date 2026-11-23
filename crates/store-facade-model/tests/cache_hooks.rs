//! Cache keys and validators.

mod support;

use store_facade_model::vocabulary::{
    SPARQL11_QUERY,
    SPARQL12_QUERY,
};
use store_facade_model::{
    CacheKey,
    EntityTag,
    FacadeError,
    QueryLanguage,
    QueryRequest,
    QueryText,
    StoreIri,
    Validator,
};

fn request(store: &str, language: QueryLanguage, text: &str) -> QueryRequest {
    QueryRequest::new(
        StoreIri::try_new(store).expect("valid"),
        language,
        QueryText::try_new(text).expect("valid"),
    )
}

fn sparql11() -> QueryLanguage {
    QueryLanguage::try_from(SPARQL11_QUERY).expect("language")
}

#[test]
fn keys_are_pinned_and_distinguish_every_field() {
    let key = request(support::STORE, sparql11(), "ASK {}").cache_key();
    assert_eq!(key.to_string().len(), 64);
    assert_eq!(
        key,
        request(support::STORE, sparql11(), "ASK {}").cache_key()
    );
    let sparql12 = QueryLanguage::try_from(SPARQL12_QUERY).expect("language");
    assert_ne!(key, request(support::STORE, sparql12, "ASK {}").cache_key());
    assert_ne!(
        key,
        request("https://example.org/other", sparql11(), "ASK {}").cache_key()
    );
    assert_ne!(
        key,
        request(support::STORE, sparql11(), "ASK { }").cache_key()
    );
    // Length prefixes keep field boundaries apart.
    assert_ne!(
        request("urn:a", sparql11(), "b c").cache_key(),
        request("urn:ab", sparql11(), "c").cache_key()
    );
    assert_eq!(
        key.to_string(),
        PINNED_KEY,
        "the v1 preimage must not change silently"
    );
}

const PINNED_KEY: &str = include_str!("fixtures/golden/cache-key.txt");

#[test]
fn keys_round_trip_as_hex() {
    let key = request(support::STORE, sparql11(), "SELECT * {}").cache_key();
    assert_eq!(key.to_string().parse::<CacheKey>().expect("parses"), key);
    let json = serde_json::to_string(&key).expect("serializes");
    assert_eq!(
        serde_json::from_str::<CacheKey>(&json).expect("parses"),
        key
    );
    for invalid in ["", "00", &"g".repeat(64), &"A".repeat(64)] {
        assert_eq!(
            invalid.parse::<CacheKey>().expect_err("invalid"),
            FacadeError::InvalidCacheKey
        );
    }
}

#[test]
fn validators_compare_as_entity_tags() {
    let tag = |text: &str| EntityTag::try_new(text).expect("valid");
    let strong = Validator::Strong(tag("v1"));
    let weak = Validator::Weak(tag("v1"));
    assert!(strong.matches_strongly(&Validator::Strong(tag("v1"))));
    assert!(!strong.matches_strongly(&weak));
    assert!(strong.matches_weakly(&weak));
    assert!(!weak.matches_weakly(&Validator::Weak(tag("v2"))));
    assert_eq!(strong.to_string(), "\"v1\"");
    assert_eq!(weak.to_string(), "W/\"v1\"");
    let json = serde_json::to_string(&weak).expect("serializes");
    assert_eq!(json, r#"{"weak":"v1"}"#);
    assert_eq!(
        serde_json::from_str::<Validator>(&json).expect("parses"),
        weak
    );
    let bytes = postcard::to_allocvec(&strong).expect("serializes");
    assert_eq!(
        postcard::from_bytes::<Validator>(&bytes).expect("parses"),
        strong
    );
}

#[test]
fn entity_tags_are_checked() {
    for invalid in ["", "a\"b", "a b", &"x".repeat(257)] {
        let error = EntityTag::try_new(invalid).expect_err("invalid");
        assert_eq!(error.code(), FacadeError::CODE_INVALID_ENTITY_TAG);
    }
    assert!(serde_json::from_str::<Validator>(r#"{"strong":"a b"}"#).is_err());
}
