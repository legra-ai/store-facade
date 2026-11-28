//! Binding, description and configuration of Oxigraph stores.

mod support;

use std::io::Cursor;

use futures_util::StreamExt;
use store_facade_model::vocabulary::{
    EXACT_LITERAL_PRESERVATION,
    NAMED_GRAPHS,
    PER_QUERY_TIMEOUT,
    RDF12_TERMS,
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
    TRANSACTIONS,
    UNION_DEFAULT_GRAPH,
};
use store_facade_model::{
    Binder,
    Capability,
    FacadeError,
    ImplementationName,
    QueryLanguage,
    QueryRequest,
    QueryStore,
    QueryText,
    StoreBinding,
    StoreIri,
    StoreSlug,
};
use store_facade_oxigraph::{
    OxigraphBinder,
    OxigraphConfig,
    OxigraphStorage,
    OxigraphStore,
    RdfFormat,
    ResultItem,
};
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

fn declared(config: OxigraphConfig) -> Vec<String> {
    let mut names: Vec<String> = support::open(config)
        .description()
        .capabilities()
        .iter()
        .map(|capability: &Capability| capability.as_str().to_owned())
        .collect();
    names.sort();
    names
}

fn names(terms: &[store_facade_model::vocabulary::CapabilityTerm]) -> Vec<String> {
    let mut names: Vec<String> = terms
        .iter()
        .map(|term| term.capability().as_str().to_owned())
        .collect();
    names.sort();
    names
}

const BASE: [store_facade_model::vocabulary::CapabilityTerm; 8] = [
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
    RDF12_TERMS,
    NAMED_GRAPHS,
    TRANSACTIONS,
    PER_QUERY_TIMEOUT,
];

#[test]
fn the_description_declares_exactly_the_supported_capabilities() {
    let plain = OxigraphConfig::new(OxigraphStorage::InMemory);
    assert_eq!(declared(plain.clone()), names(&BASE));
    let mut with_union = BASE.to_vec();
    with_union.push(UNION_DEFAULT_GRAPH);
    assert_eq!(
        declared(plain.with_union_default_graph()),
        names(&with_union)
    );
    assert!(
        !declared(OxigraphConfig::new(OxigraphStorage::InMemory))
            .contains(&EXACT_LITERAL_PRESERVATION.capability().as_str().to_owned())
    );
}

#[test]
fn the_description_names_no_technology() {
    let directory = TempDir::new().expect("a temporary directory");
    let store = support::open(OxigraphConfig::new(OxigraphStorage::on_disk(
        directory.path(),
    )));
    let rendered = format!("{:?}", store.description().triples().collect::<Vec<_>>());
    for leak in [
        "oxigraph",
        "Oxigraph",
        "rocksdb",
        directory.path().to_str().expect("utf-8"),
    ] {
        assert!(!rendered.contains(leak), "{leak} in {rendered}");
    }
}

#[test]
fn a_binding_of_another_implementation_is_refused() {
    let binding = StoreBinding::try_new(
        StoreIri::try_new(support::STORE).expect("valid"),
        StoreSlug::try_new("sales").expect("valid"),
        support::labels(),
        ImplementationName::try_new("other").expect("valid"),
        OxigraphConfig::new(OxigraphStorage::InMemory),
    )
    .expect("valid binding");
    assert_eq!(
        OxigraphBinder.bind(binding).expect_err("mismatch").code(),
        FacadeError::CODE_BINDING_MISMATCH
    );
}

#[test]
fn the_configuration_has_a_stable_serialized_form() {
    let json = r#"{"storage":{"kind":"on-disk","path":"/srv/sales"},"union_default_graph":true,"result_buffer":8}"#;
    let config: OxigraphConfig = serde_json::from_str(json).expect("parses");
    assert_eq!(config.storage(), &OxigraphStorage::on_disk("/srv/sales"));
    assert!(config.union_default_graph());
    assert_eq!(config.result_buffer().items(), 8);
    assert_eq!(serde_json::to_string(&config).expect("serializes"), json);
    let minimal: OxigraphConfig =
        serde_json::from_str(r#"{"storage":{"kind":"in-memory"}}"#).expect("defaults apply");
    assert_eq!(minimal, OxigraphConfig::new(OxigraphStorage::InMemory));
    assert!(
        serde_json::from_str::<OxigraphConfig>(
            r#"{"storage":{"kind":"in-memory"},"result_buffer":0}"#
        )
        .is_err()
    );
}

fn request(language: store_facade_model::vocabulary::CapabilityTerm, text: &str) -> QueryRequest {
    QueryRequest::new(
        StoreIri::try_new(support::STORE).expect("valid"),
        QueryLanguage::try_from(language).expect("a language"),
        QueryText::try_new(text).expect("non-empty"),
    )
}

async fn ask(store: &OxigraphStore, text: &str) -> bool {
    let mut stream = store
        .query(request(SPARQL11_QUERY, text), CancellationToken::new())
        .await
        .expect("starts");
    match stream.next().await.expect("an answer").expect("no error") {
        ResultItem::Boolean(answer) => answer,
        other => panic!("expected a boolean, got {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn on_disk_data_survives_reopening() {
    let directory = TempDir::new().expect("a temporary directory");
    let config = OxigraphConfig::new(OxigraphStorage::on_disk(directory.path()));
    {
        let store = support::open(config.clone());
        let applied = store
            .query(
                request(SPARQL11_UPDATE, "INSERT DATA { <urn:s> <urn:p> <urn:o> }"),
                CancellationToken::new(),
            )
            .await
            .expect("updates");
        assert_eq!(applied.count().await, 0);
    }
    let reopened = support::open(config);
    assert!(ask(&reopened, "ASK { <urn:s> <urn:p> <urn:o> }").await);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_literals_return_canonical_which_is_why_exactness_is_not_declared() {
    let store = support::open(OxigraphConfig::new(OxigraphStorage::InMemory));
    store
        .load(
            RdfFormat::NQuads,
            Cursor::new(
                b"<urn:s> <urn:p> \"01\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n".to_vec(),
            ),
        )
        .await
        .expect("seeds");
    let mut stream = store
        .query(
            request(SPARQL11_QUERY, "SELECT ?o WHERE { <urn:s> <urn:p> ?o }"),
            CancellationToken::new(),
        )
        .await
        .expect("starts");
    let Some(Ok(ResultItem::Solution(solution))) = stream.next().await else {
        panic!("expected a solution");
    };
    let object = solution.get("o").expect("bound").to_string();
    assert_eq!(object, "\"1\"^^<http://www.w3.org/2001/XMLSchema#integer>");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn errors_carry_the_model_codes() {
    let store = support::open(OxigraphConfig::new(OxigraphStorage::InMemory));
    let failing_update = store
        .query(
            request(SPARQL11_UPDATE, "CLEAR GRAPH <urn:absent>"),
            CancellationToken::new(),
        )
        .await
        .expect_err("clearing an absent graph fails");
    assert_eq!(failing_update.code(), FacadeError::CODE_BACKEND);
    let bad_data = store
        .load(RdfFormat::NQuads, Cursor::new(b"not n-quads".to_vec()))
        .await
        .expect_err("unparsable data fails");
    assert_eq!(bad_data.code(), FacadeError::CODE_BACKEND);
    let syntax = store
        .query(request(SPARQL11_QUERY, "ASK {"), CancellationToken::new())
        .await
        .expect_err("a syntax error fails");
    assert_eq!(syntax.code(), FacadeError::CODE_INVALID_QUERY);
}
