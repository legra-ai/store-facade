use store_facade_model::vocabulary::TRANSACTIONS;

use crate::{
    ContractBinding,
    Probe,
};

/// A multi-operation update whose last operation fails leaves no trace.
pub(crate) async fn transactions<B: ContractBinding>(probe: &Probe<'_, B>) {
    let query = probe.query_language(TRANSACTIONS);
    let update = probe.update_language(TRANSACTIONS);
    let outcome = probe
        .run(probe.request(
            update,
            "INSERT DATA { <urn:s> <urn:p> <urn:o> } ; CREATE GRAPH <urn:g> ; CREATE GRAPH <urn:g>",
        ))
        .await;
    assert!(outcome.is_err(), "creating an existing graph fails");
    assert!(!probe.ask(query, "ASK { <urn:s> <urn:p> <urn:o> }").await);
    assert!(!probe.ask(query, "ASK { GRAPH <urn:g> { ?s ?p ?o } }").await);
}
