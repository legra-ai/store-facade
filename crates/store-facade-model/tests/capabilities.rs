//! Capability sets and requirement checks.

use store_facade_model::vocabulary::{
    ALL_CAPABILITIES,
    FULL_TEXT_SEARCH,
    GEOSPARQL,
    NAMED_GRAPHS,
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
};
use store_facade_model::{
    Capability,
    CapabilityRole,
    CapabilitySet,
    DescriptionLimit,
    FacadeError,
    QueryLanguage,
    RequiredCapabilities,
};

fn set(terms: &[store_facade_model::vocabulary::CapabilityTerm]) -> CapabilitySet {
    CapabilitySet::try_from_iter(terms.iter().map(|term| term.capability())).expect("small set")
}

#[test]
fn require_passes_when_every_capability_is_present() {
    let store = set(&[SPARQL11_QUERY, NAMED_GRAPHS, GEOSPARQL]);
    let required = RequiredCapabilities::single(SPARQL11_QUERY)
        .with(GEOSPARQL)
        .expect("small");
    store.require(&required).expect("satisfied");
    store
        .require(&RequiredCapabilities::default())
        .expect("nothing required");
}

#[test]
fn require_names_every_missing_capability() {
    let store = set(&[SPARQL11_QUERY]);
    let required = RequiredCapabilities::try_from_iter(
        [SPARQL11_QUERY, SPARQL11_UPDATE, FULL_TEXT_SEARCH].map(Capability::from),
    )
    .expect("small");
    let error = store.require(&required).expect_err("missing two");
    assert_eq!(error.code(), FacadeError::CODE_MISSING_CAPABILITIES);
    let FacadeError::MissingCapabilities(missing) = &error else {
        panic!("{error:?}")
    };
    assert_eq!(
        missing.capabilities(),
        &set(&[SPARQL11_UPDATE, FULL_TEXT_SEARCH])
    );
    assert_eq!(
        error.to_string(),
        "the store lacks required capabilities: \
         <http://www.w3.org/ns/sparql-service-description#SPARQL11Update> \
         <https://ns.legra.ai/store-facade#FullTextSearch>"
    );
}

#[test]
fn roles_are_part_of_identity() {
    let iri = NAMED_GRAPHS.iri();
    let feature = Capability::try_new(CapabilityRole::Feature, iri).expect("valid");
    let language = Capability::try_new(CapabilityRole::Language, iri).expect("valid");
    assert_ne!(feature, language);
    let mut capabilities = CapabilitySet::new();
    assert!(capabilities.insert(feature.clone()).expect("room"));
    assert!(!capabilities.insert(feature).expect("duplicate"));
    assert!(capabilities.insert(language).expect("room"));
    assert_eq!(capabilities.len(), 2);
}

#[test]
fn capability_iris_are_checked() {
    let error = Capability::try_new(CapabilityRole::Feature, "no scheme").expect_err("invalid");
    assert_eq!(error.code(), FacadeError::CODE_INVALID_CAPABILITY);
}

#[test]
fn query_languages_must_have_the_language_role() {
    QueryLanguage::try_from(SPARQL11_QUERY).expect("a language");
    let error = QueryLanguage::try_from(NAMED_GRAPHS).expect_err("a feature");
    assert_eq!(error.code(), FacadeError::CODE_CAPABILITY_ROLE_MISMATCH);
    assert!(
        serde_json::from_str::<QueryLanguage>(
            r#"{"role":"feature","iri":"https://ns.legra.ai/store-facade#NamedGraphs"}"#
        )
        .is_err()
    );
}

#[test]
fn capability_sets_are_bounded() {
    let limit = DescriptionLimit::Capabilities.max();
    let many = (0..=limit).map(|index| {
        Capability::try_new(CapabilityRole::Feature, format!("urn:feature:{index}")).expect("valid")
    });
    let error = CapabilitySet::try_from_iter(many).expect_err("one too many");
    assert_eq!(
        error,
        FacadeError::LimitExceeded {
            limit: DescriptionLimit::Capabilities
        }
    );
    let json = serde_json::to_string(
        &(0..=limit)
            .map(|index| {
                Capability::try_new(CapabilityRole::Feature, format!("urn:f:{index}")).expect("ok")
            })
            .collect::<Vec<_>>(),
    )
    .expect("serializes");
    assert!(serde_json::from_str::<CapabilitySet>(&json).is_err());
}

#[test]
fn the_catalogue_is_a_valid_set() {
    let catalogue =
        CapabilitySet::try_from_iter(ALL_CAPABILITIES.map(Capability::from)).expect("fits");
    assert_eq!(catalogue.len(), ALL_CAPABILITIES.len());
}
