use store_facade_model::FacadeError;
use store_facade_model::vocabulary::{
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
};

use crate::{
    ContractBinding,
    Probe,
};

/// INSERT DATA and DELETE DATA change what queries see; a syntax error is
/// `invalid-query`.
pub(crate) async fn sparql11_update<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(SPARQL11_UPDATE);
    probe
        .update(SPARQL11_UPDATE, "INSERT DATA { <urn:s> <urn:p> <urn:o> }")
        .await;
    assert!(probe.ask(language, "ASK { <urn:s> <urn:p> <urn:o> }").await);
    probe
        .update(SPARQL11_UPDATE, "DELETE DATA { <urn:s> <urn:p> <urn:o> }")
        .await;
    assert!(!probe.ask(language, "ASK { <urn:s> <urn:p> <urn:o> }").await);
    let error = probe
        .run(probe.request(SPARQL11_UPDATE, "INSERT DATUM { <urn:s> <urn:p> <urn:o> }"))
        .await
        .expect_err("a syntax error fails");
    assert_eq!(error.code(), FacadeError::CODE_INVALID_QUERY, "{error}");
}

/// A SPARQL 1.2 update stores a triple term that SPARQL 1.2 queries match.
pub(crate) async fn sparql12_update<B: ContractBinding>(probe: &Probe<'_, B>) {
    assert!(
        probe.declares(SPARQL12_QUERY),
        "sf:SPARQL12Update is only observable through sf:SPARQL12Query"
    );
    probe
        .update(
            SPARQL12_UPDATE,
            "INSERT DATA { <urn:s> <urn:p> <<( <urn:a> <urn:b> <urn:c> )>> }",
        )
        .await;
    assert!(
        probe
            .ask(
                SPARQL12_QUERY,
                "ASK { <urn:s> <urn:p> <<( <urn:a> <urn:b> <urn:c> )>> }",
            )
            .await
    );
}
