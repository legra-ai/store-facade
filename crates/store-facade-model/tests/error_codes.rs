//! The error-code catalogue is a stable public contract.

use std::collections::BTreeSet;

use store_facade_model::{
    ErrorCode,
    FacadeError,
};

#[test]
fn the_catalogue_is_pinned() {
    let codes: Vec<&str> = FacadeError::CODES
        .iter()
        .map(|code| code.as_str())
        .collect();
    assert_eq!(
        codes,
        [
            "invalid-store-iri",
            "invalid-store-slug",
            "invalid-capability",
            "capability-role-mismatch",
            "invalid-label",
            "missing-label",
            "missing-slug",
            "limit-exceeded",
            "missing-capabilities",
            "invalid-description-rdf",
            "store-mismatch",
            "empty-query",
            "invalid-entity-tag",
            "invalid-cache-key",
            "cancelled",
            "backend",
        ]
    );
    let unique: BTreeSet<ErrorCode> = FacadeError::CODES.into_iter().collect();
    assert_eq!(unique.len(), FacadeError::CODES.len());
}

#[test]
fn codes_are_kebab_case_and_serialize_as_strings() {
    for code in FacadeError::CODES {
        assert!(
            code.as_str()
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b == b'-'),
            "{code}"
        );
        assert_eq!(
            serde_json::to_string(&code).expect("serializes"),
            format!("\"{code}\"")
        );
    }
}

#[test]
fn variants_map_to_their_codes() {
    assert_eq!(FacadeError::Cancelled.code(), FacadeError::CODE_CANCELLED);
    assert_eq!(
        FacadeError::backend("timed out").code(),
        FacadeError::CODE_BACKEND
    );
    assert_eq!(
        FacadeError::EmptyQuery.code(),
        FacadeError::CODE_EMPTY_QUERY
    );
    assert_eq!(
        FacadeError::MissingSlug.code(),
        FacadeError::CODE_MISSING_SLUG
    );
    assert_eq!(
        FacadeError::InvalidCacheKey.code(),
        FacadeError::CODE_INVALID_CACHE_KEY
    );
}
