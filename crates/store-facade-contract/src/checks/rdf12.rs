use store_facade_model::vocabulary::RDF12_TERMS;

use crate::{
    ContractBinding,
    Probe,
    SeedQuads,
};

/// Triple terms and directional language-tagged literals survive storage.
pub(crate) async fn rdf12_terms<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(RDF12_TERMS);
    probe
        .seed(&SeedQuads::new(concat!(
            "<urn:s> <urn:p1> <<( <urn:a> <urn:b> <urn:c> )>> .\n",
            "<urn:s> <urn:p2> \"shalom\"@he--rtl .\n",
        )))
        .await;
    let objects = probe
        .select(
            language,
            "SELECT ?o WHERE { <urn:s> ?p ?o } ORDER BY ?p",
            "o",
        )
        .await;
    assert_eq!(
        objects,
        ["<<( <urn:a> <urn:b> <urn:c> )>>", "\"shalom\"@he--rtl"]
    );
}
