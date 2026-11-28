use store_facade_model::FacadeError;
use store_facade_model::vocabulary::{
    SPARQL11_QUERY,
    SPARQL12_QUERY,
};

use crate::{
    ContractBinding,
    Observation,
    Probe,
    SeedQuads,
    TermText,
};

/// SELECT, ASK and CONSTRUCT answer over seeded data; a syntax error is
/// `invalid-query`.
pub(crate) async fn sparql11_query<B: ContractBinding>(probe: &Probe<'_, B>) {
    probe
        .seed(&SeedQuads::new("<urn:s> <urn:p> \"o\" .\n"))
        .await;
    let objects = probe
        .select(
            SPARQL11_QUERY,
            "SELECT ?o WHERE { <urn:s> <urn:p> ?o }",
            "o",
        )
        .await;
    assert_eq!(objects, ["\"o\""]);
    assert!(
        probe
            .ask(SPARQL11_QUERY, "ASK { <urn:s> <urn:p> \"o\" }")
            .await
    );
    assert!(
        !probe
            .ask(SPARQL11_QUERY, "ASK { <urn:s> <urn:p> \"other\" }")
            .await
    );
    let triples = probe
        .run(probe.request(
            SPARQL11_QUERY,
            "CONSTRUCT { ?s <urn:q> ?o } WHERE { ?s <urn:p> ?o }",
        ))
        .await
        .expect("CONSTRUCT answers");
    assert_eq!(
        triples,
        [Observation::Triple(
            TermText::new("<urn:s>"),
            TermText::new("<urn:q>"),
            TermText::new("\"o\""),
        )]
    );
    let error = probe
        .run(probe.request(SPARQL11_QUERY, "SELEKT * WHERE { ?s ?p ?o }"))
        .await
        .expect_err("a syntax error fails");
    assert_eq!(error.code(), FacadeError::CODE_INVALID_QUERY, "{error}");
}

/// A SPARQL 1.2 query builds and returns a triple term.
pub(crate) async fn sparql12_query<B: ContractBinding>(probe: &Probe<'_, B>) {
    probe
        .seed(&SeedQuads::new("<urn:s> <urn:p> <urn:o> .\n"))
        .await;
    let terms = probe
        .select(
            SPARQL12_QUERY,
            "SELECT ?t WHERE { ?s <urn:p> ?o BIND(<<( ?s <urn:p> ?o )>> AS ?t) }",
            "t",
        )
        .await;
    assert_eq!(terms, ["<<( <urn:s> <urn:p> <urn:o> )>>"]);
}
