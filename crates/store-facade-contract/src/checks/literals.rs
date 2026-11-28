use store_facade_model::vocabulary::EXACT_LITERAL_PRESERVATION;

use crate::{
    ContractBinding,
    Probe,
    SeedQuads,
};

/// Literals return with the lexical form they were stored with.
pub(crate) async fn exact_literal_preservation<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(EXACT_LITERAL_PRESERVATION);
    probe
        .seed(&SeedQuads::new(concat!(
            "<urn:s> <urn:p1> \"01\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n",
            "<urn:s> <urn:p2> \"+1.50\"^^<http://www.w3.org/2001/XMLSchema#decimal> .\n",
            "<urn:s> <urn:p3> \"true \"^^<http://www.w3.org/2001/XMLSchema#string> .\n",
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
        [
            "\"01\"^^<http://www.w3.org/2001/XMLSchema#integer>",
            "\"+1.50\"^^<http://www.w3.org/2001/XMLSchema#decimal>",
            "\"true \"",
        ]
    );
}
