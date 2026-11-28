use store_facade_model::vocabulary::{
    NAMED_GRAPHS,
    UNION_DEFAULT_GRAPH,
};

use crate::{
    ContractBinding,
    Probe,
    SeedQuads,
};

const SEED: &str = concat!(
    "<urn:s> <urn:p> <urn:o> <urn:g> .\n",
    "<urn:d> <urn:p> <urn:o> .\n",
);

/// `GRAPH ?g` finds the named graph a quad was seeded in, and nothing of the
/// default graph; without a union default graph, the default graph holds
/// only its own triples.
pub(crate) async fn named_graphs<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(NAMED_GRAPHS);
    probe.seed(&SeedQuads::new(SEED)).await;
    let graphs = probe
        .select(
            language,
            "SELECT ?g WHERE { GRAPH ?g { ?s <urn:p> <urn:o> } }",
            "g",
        )
        .await;
    assert_eq!(graphs, ["<urn:g>"]);
    if !probe.declares(UNION_DEFAULT_GRAPH) {
        assert!(!probe.ask(language, "ASK { <urn:s> <urn:p> <urn:o> }").await);
        assert!(probe.ask(language, "ASK { <urn:d> <urn:p> <urn:o> }").await);
    }
}

/// The default graph is the union of the named graphs.
pub(crate) async fn union_default_graph<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(UNION_DEFAULT_GRAPH);
    probe.seed(&SeedQuads::new(SEED)).await;
    assert!(probe.ask(language, "ASK { <urn:s> <urn:p> <urn:o> }").await);
}
