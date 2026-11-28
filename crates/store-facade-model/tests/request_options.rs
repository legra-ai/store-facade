//! Request-level capability requirements and per-query timeouts.

mod support;

use core::time::Duration;

use store_facade_model::vocabulary::{
    EXACT_LITERAL_PRESERVATION,
    GEOSPARQL,
    NAMED_GRAPHS,
    PER_QUERY_TIMEOUT,
    SPARQL11_QUERY,
};
use store_facade_model::{
    Capability,
    CapabilityRole,
    FacadeError,
    QueryLanguage,
    QueryRequest,
    QueryText,
    QueryTimeout,
};

fn request() -> QueryRequest {
    QueryRequest::new(
        support::store_iri(),
        QueryLanguage::try_from(SPARQL11_QUERY).expect("a language"),
        QueryText::try_new("ASK {}").expect("non-empty"),
    )
}

#[test]
fn a_required_feature_joins_the_language_requirement() {
    let request = request().requiring(NAMED_GRAPHS).expect("within limit");
    let required = request.required_capabilities();
    let required: Vec<&str> = required.iter().map(Capability::as_str).collect();
    assert_eq!(
        required,
        [
            SPARQL11_QUERY.capability().as_str(),
            NAMED_GRAPHS.capability().as_str()
        ]
    );
    request
        .check_against(&support::sample())
        .expect("the sample declares named graphs");
}

#[test]
fn an_undeclared_feature_is_refused_by_name() {
    let request = request()
        .requiring(EXACT_LITERAL_PRESERVATION)
        .expect("within limit")
        .requiring(GEOSPARQL)
        .expect("within limit");
    let Err(FacadeError::MissingCapabilities(missing)) = request.check_against(&support::sample())
    else {
        panic!("expected a missing-capabilities refusal");
    };
    assert!(
        missing
            .capabilities()
            .contains(&EXACT_LITERAL_PRESERVATION.capability())
    );
    assert!(missing.capabilities().contains(&GEOSPARQL.capability()));
}

#[test]
fn a_timeout_requires_per_query_timeout() {
    let timeout = QueryTimeout::try_from_millis(250).expect("in range");
    let request = request().with_timeout(timeout);
    assert_eq!(request.timeout(), Some(timeout));
    assert!(
        request
            .required_capabilities()
            .as_set()
            .contains(&PER_QUERY_TIMEOUT.capability())
    );
    let error = request
        .check_against(&support::sample())
        .expect_err("the sample has no per-query timeout");
    assert_eq!(error.code(), FacadeError::CODE_MISSING_CAPABILITIES);
}

#[test]
fn required_features_are_bounded() {
    let mut request = request();
    for index in 0..QueryRequest::MAX_REQUIRED_FEATURES {
        let feature = Capability::try_new(
            CapabilityRole::Feature,
            format!("https://example.org/feature/{index}"),
        )
        .expect("valid IRI");
        request = request.requiring(feature).expect("within limit");
    }
    let again = Capability::try_new(CapabilityRole::Feature, "https://example.org/feature/0")
        .expect("valid IRI");
    let request = request.requiring(again).expect("a duplicate adds nothing");
    let extra = Capability::try_new(CapabilityRole::Feature, "https://example.org/feature/x")
        .expect("valid IRI");
    assert_eq!(
        request.requiring(extra).expect_err("over the limit").code(),
        FacadeError::CODE_LIMIT_EXCEEDED
    );
}

#[test]
fn timeouts_are_range_checked_and_serialize_as_milliseconds() {
    for millis in [0, QueryTimeout::MAX_MILLIS + 1] {
        assert_eq!(
            QueryTimeout::try_from_millis(millis)
                .expect_err("out of range")
                .code(),
            FacadeError::CODE_INVALID_QUERY_TIMEOUT
        );
    }
    let timeout = QueryTimeout::try_from_duration(Duration::from_micros(1_500)).expect("in range");
    assert_eq!(timeout.as_millis(), 1);
    assert_eq!(timeout.as_duration(), Duration::from_millis(1));
    assert_eq!(serde_json::to_string(&timeout).expect("serializes"), "1");
    assert!(serde_json::from_str::<QueryTimeout>("0").is_err());
}

#[test]
fn options_survive_serialization() {
    let request = request()
        .requiring(NAMED_GRAPHS)
        .expect("within limit")
        .with_timeout(QueryTimeout::try_from_millis(5).expect("in range"));
    let bytes = postcard::to_allocvec(&request).expect("serializes");
    assert_eq!(
        postcard::from_bytes::<QueryRequest>(&bytes).expect("parses"),
        request
    );
    let json = serde_json::to_string(&request).expect("serializes");
    assert_eq!(
        serde_json::from_str::<QueryRequest>(&json).expect("parses"),
        request
    );
}

#[test]
fn options_do_not_change_the_cache_key() {
    let plain = request();
    let optioned = plain
        .clone()
        .requiring(NAMED_GRAPHS)
        .expect("within limit")
        .with_timeout(QueryTimeout::try_from_millis(5).expect("in range"));
    assert_eq!(plain.cache_key(), optioned.cache_key());
}
