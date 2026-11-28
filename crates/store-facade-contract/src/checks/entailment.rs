use store_facade_model::vocabulary::{
    ENTAILMENT_OWL_RDF_BASED,
    ENTAILMENT_RDFS,
};

use crate::{
    ContractBinding,
    Probe,
    SeedQuads,
};

/// Membership follows `rdfs:subClassOf`.
pub(crate) async fn rdfs<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(ENTAILMENT_RDFS);
    probe
        .seed(&SeedQuads::new(concat!(
            "<urn:a> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <urn:C> .\n",
            "<urn:C> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <urn:D> .\n",
        )))
        .await;
    assert!(
        probe
            .ask(
                language,
                "ASK { <urn:a> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <urn:D> }",
            )
            .await
    );
}

/// Statements follow `owl:inverseOf`.
pub(crate) async fn owl_rdf_based<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(ENTAILMENT_OWL_RDF_BASED);
    probe
        .seed(&SeedQuads::new(concat!(
            "<urn:p> <http://www.w3.org/2002/07/owl#inverseOf> <urn:q> .\n",
            "<urn:a> <urn:p> <urn:b> .\n",
        )))
        .await;
    assert!(probe.ask(language, "ASK { <urn:b> <urn:q> <urn:a> }").await);
}
