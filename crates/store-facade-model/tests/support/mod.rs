//! Shared test fixtures.
#![allow(dead_code)]

use rdf_structural_model::{
    LanguageTag,
    LiteralFacets,
    Subject,
    Term,
    Triple,
    XSD_STRING,
};
use store_facade_model::vocabulary::{
    ENTAILMENT_RDFS,
    NAMED_GRAPHS,
    SPARQL11_QUERY,
    SPARQL12_QUERY,
};
use store_facade_model::{
    StoreDescription,
    StoreIri,
    StoreSlug,
};

pub const STORE: &str = "https://example.org/store/sales";

pub fn tag(text: &str) -> LanguageTag {
    LanguageTag::try_new(text.to_owned()).expect("valid language tag")
}

pub fn store_iri() -> StoreIri {
    StoreIri::try_new(STORE).expect("valid Store IRI")
}

pub fn sample() -> StoreDescription {
    StoreDescription::builder(
        store_iri(),
        StoreSlug::try_new("sales").expect("valid slug"),
    )
    .label(tag("en"), "Sales ledger")
    .label(tag("nl"), "Verkoopgrootboek")
    .capabilities([
        SPARQL11_QUERY,
        SPARQL12_QUERY,
        NAMED_GRAPHS,
        ENTAILMENT_RDFS,
    ])
    .build()
    .expect("valid description")
}

/// Renders triples as N-Triples, one per line, for golden comparison.
pub fn n_triples(triples: impl IntoIterator<Item = Triple>) -> String {
    triples.into_iter().fold(String::new(), |mut text, triple| {
        text.push_str(&line(&triple));
        text.push('\n');
        text
    })
}

fn line(triple: &Triple) -> String {
    let Subject::Iri(subject) = triple.subject() else {
        panic!("IRI subjects only")
    };
    format!(
        "<{subject}> <{}> {} .",
        triple.predicate(),
        object(triple.object())
    )
}

fn object(term: &Term) -> String {
    match term {
        Term::Iri(iri) => format!("<{iri}>"),
        Term::Literal(literal) => {
            let lexical = escape(literal.lexical());
            match literal.metadata().facets() {
                LiteralFacets::Language(language) => format!("\"{lexical}\"@{}", language.as_str()),
                LiteralFacets::None if literal.metadata().datatype().as_str() == XSD_STRING => {
                    format!("\"{lexical}\"")
                }
                LiteralFacets::None => {
                    format!("\"{lexical}\"^^<{}>", literal.metadata().datatype())
                }
                LiteralFacets::Directional { .. } => panic!("no directional literals expected"),
            }
        }
        _ => panic!("IRIs and literals only"),
    }
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
