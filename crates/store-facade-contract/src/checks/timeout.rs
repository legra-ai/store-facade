use core::fmt::Write as _;
use core::time::Duration;

use store_facade_model::FacadeError;
use store_facade_model::vocabulary::PER_QUERY_TIMEOUT;

use crate::{
    ContractBinding,
    Probe,
    SeedQuads,
};

/// Seeded triples; a five-deep chain of correlated `OPTIONAL`s over them
/// never finishes in time.
const TRIPLES: usize = 64;
/// How long a timed-out query may take to end, timeout included.
const DEADLINE: Duration = Duration::from_secs(20);

/// A query that outruns its timeout ends with `timeout`; one that does not
/// completes normally.
pub(crate) async fn per_query_timeout<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(PER_QUERY_TIMEOUT);
    let seed = (0..TRIPLES).fold(String::new(), |mut text, index| {
        let _ = writeln!(text, "<urn:s{index}> <urn:p> <urn:o{index}> .");
        text
    });
    probe.seed(&SeedQuads::new(seed)).await;
    let quick = probe
        .run(probe.timed_request(language, "ASK { <urn:s0> <urn:p> <urn:o0> }", 60_000))
        .await
        .expect("a quick query completes within its timeout");
    assert_eq!(quick.len(), 1);
    let slow = probe.timed_request(
        language,
        concat!(
            "SELECT (COUNT(*) AS ?n) WHERE { ?a ?p ?b OPTIONAL { ?c ?q ?d FILTER(?c != ?a) ",
            "OPTIONAL { ?e ?r ?f FILTER(?e != ?c) OPTIONAL { ?g ?s ?h FILTER(?g != ?e) ",
            "OPTIONAL { ?i ?t ?j FILTER(?i != ?g) } } } } }",
        ),
        50,
    );
    let error = tokio::time::timeout(DEADLINE, probe.run(slow))
        .await
        .expect("a timed-out query ends promptly")
        .expect_err("the cross product outruns its timeout");
    assert_eq!(error, FacadeError::Timeout);
}
