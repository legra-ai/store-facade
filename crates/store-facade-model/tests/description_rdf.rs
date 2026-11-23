//! RDF reading and writing of store descriptions.

mod support;

use iri_rfc3987::Iri;
use rdf_structural_model::{
    Literal,
    Subject,
    Term,
    Triple,
};
use store_facade_model::vocabulary::{
    RDF_TYPE,
    RDFS_LABEL,
    SD_FEATURE,
    SF_SLUG,
    SF_STORE,
};
use store_facade_model::{
    FacadeError,
    StoreDescription,
    StoreDescriptionReader,
    StoreIri,
};
use support::{
    STORE,
    n_triples,
    sample,
    store_iri,
    tag,
};

const GOLDEN: &str = include_str!("fixtures/golden/sales.nt");

fn triple(subject: &str, predicate: &str, object: Term) -> Triple {
    Triple::new(
        Subject::Iri(Iri::known(subject)),
        Iri::known(predicate),
        object,
    )
}

fn read(triples: &[Triple]) -> Result<StoreDescription, FacadeError> {
    StoreDescription::from_triples(store_iri(), triples)
}

fn rdf_error(triples: &[Triple]) -> FacadeError {
    read(triples).expect_err("rejected")
}

fn base() -> Vec<Triple> {
    vec![
        triple(STORE, RDF_TYPE, Term::iri(Iri::known(SF_STORE))),
        triple(STORE, SF_SLUG, Term::literal(Literal::string("sales"))),
        triple(
            STORE,
            RDFS_LABEL,
            Term::literal(Literal::language_tagged("Sales", tag("en"))),
        ),
    ]
}

#[test]
fn writes_the_golden_triples() {
    assert_eq!(n_triples(sample().triples()), GOLDEN);
}

#[test]
fn round_trips_in_any_order_among_other_subjects() {
    let description = sample();
    let mut triples: Vec<Triple> = description.triples().collect();
    triples.reverse();
    triples.insert(
        1,
        triple(
            "https://example.org/other",
            RDF_TYPE,
            Term::iri(Iri::known(SF_STORE)),
        ),
    );
    triples.push(triple(
        "https://example.org/other",
        "urn:anything",
        Term::literal(Literal::string("x")),
    ));
    assert_eq!(read(&triples).expect("valid"), description);
}

#[test]
fn restated_triples_are_idempotent() {
    let description = sample();
    let once: Vec<Triple> = description.triples().collect();
    let twice: Vec<Triple> = once.iter().chain(once.iter()).cloned().collect();
    assert_eq!(read(&twice).expect("valid"), description);
}

#[test]
fn accept_reports_whether_a_triple_was_about_the_store() {
    let mut reader = StoreDescriptionReader::new(store_iri());
    let other = triple(
        "https://example.org/other",
        "urn:p",
        Term::iri(Iri::known("urn:o")),
    );
    assert!(!reader.accept(&other).expect("skipped"));
    assert!(reader.accept(&base()[0]).expect("accepted"));
}

#[test]
fn requires_type_slug_and_label() {
    let triples = base();
    let reason = |error| match error {
        FacadeError::InvalidDescriptionRdf { .. } => "rdf",
        FacadeError::MissingSlug => "slug",
        FacadeError::MissingLabel => "label",
        other => panic!("{other:?}"),
    };
    assert_eq!(reason(rdf_error(&triples[1..])), "rdf");
    assert_eq!(
        reason(rdf_error(&[triples[0].clone(), triples[2].clone()])),
        "slug"
    );
    assert_eq!(reason(rdf_error(&triples[..2])), "label");
}

#[test]
fn rejects_statements_a_description_cannot_hold() {
    let reject = |extra: Triple| {
        let mut triples = base();
        triples.push(extra);
        rdf_error(&triples).code()
    };
    let rdf = FacadeError::CODE_INVALID_DESCRIPTION_RDF;
    assert_eq!(
        reject(triple(
            STORE,
            RDF_TYPE,
            Term::iri(Iri::known("urn:product:Engine"))
        )),
        rdf
    );
    assert_eq!(
        reject(triple(
            STORE,
            "urn:cluster:region",
            Term::literal(Literal::string("eu"))
        )),
        rdf
    );
    assert_eq!(
        reject(triple(
            STORE,
            SF_SLUG,
            Term::literal(Literal::string("sales-2"))
        )),
        rdf
    );
    assert_eq!(
        reject(triple(STORE, SF_SLUG, Term::iri(Iri::known("urn:sales")))),
        rdf
    );
    assert_eq!(
        reject(triple(
            STORE,
            SD_FEATURE,
            Term::literal(Literal::string("named"))
        )),
        rdf
    );
    assert_eq!(
        reject(triple(
            STORE,
            RDFS_LABEL,
            Term::literal(Literal::string("untagged"))
        )),
        FacadeError::CODE_INVALID_LABEL
    );
    assert_eq!(
        reject(triple(
            STORE,
            RDFS_LABEL,
            Term::literal(Literal::language_tagged("Revenue", tag("en")))
        )),
        FacadeError::CODE_INVALID_LABEL
    );
    let typed_slug = Literal::typed("sales", Iri::known("urn:datatype")).expect("valid literal");
    let mut triples = vec![
        base()[0].clone(),
        triple(STORE, SF_SLUG, Term::literal(typed_slug)),
    ];
    triples.push(base()[2].clone());
    assert_eq!(rdf_error(&triples).code(), rdf);
    let bad_slug = vec![
        base()[0].clone(),
        triple(STORE, SF_SLUG, Term::literal(Literal::string("Bad Slug"))),
    ];
    assert_eq!(
        rdf_error(&bad_slug).code(),
        FacadeError::CODE_INVALID_STORE_SLUG
    );
}

#[test]
fn reads_only_the_requested_store() {
    let other = StoreIri::try_new("https://example.org/other").expect("valid");
    let triples: Vec<Triple> = sample().triples().collect();
    let error = StoreDescription::from_triples(other, &triples).expect_err("nothing about it");
    assert_eq!(error.code(), FacadeError::CODE_INVALID_DESCRIPTION_RDF);
}
