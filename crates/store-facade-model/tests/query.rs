//! The streaming, cancellable query contract.

mod support;

use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};

use futures_util::stream::{
    self,
    BoxStream,
    StreamExt,
};
use store_facade_model::vocabulary::{
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
};
use store_facade_model::{
    FacadeError,
    QueryLanguage,
    QueryRequest,
    QueryStore,
    QueryText,
    StoreDescription,
    StoreIri,
};
use tokio_util::sync::CancellationToken;

/// Yields each row of its query text's words, then stalls if asked to.
struct WordStore {
    description: StoreDescription,
    stall: bool,
    executed: AtomicUsize,
}

impl WordStore {
    fn new(stall: bool) -> Self {
        Self {
            description: support::sample(),
            stall,
            executed: AtomicUsize::new(0),
        }
    }
}

impl QueryStore for WordStore {
    type Item = String;
    type Stream = BoxStream<'static, Result<String, FacadeError>>;

    fn description(&self) -> &StoreDescription {
        &self.description
    }

    async fn execute(
        &self,
        request: QueryRequest,
        _cancellation: CancellationToken,
    ) -> Result<Self::Stream, FacadeError> {
        self.executed.fetch_add(1, Ordering::SeqCst);
        // A real backend awaits its connection here.
        tokio::task::yield_now().await;
        let words: Vec<Result<String, FacadeError>> = request
            .text()
            .as_str()
            .split_whitespace()
            .map(|word| Ok(word.to_owned()))
            .collect();
        let rows = stream::iter(words);
        Ok(if self.stall {
            rows.chain(stream::pending()).boxed()
        } else {
            rows.boxed()
        })
    }
}

fn request(store: StoreIri, language: QueryLanguage) -> QueryRequest {
    QueryRequest::new(
        store,
        language,
        QueryText::try_new("one two three").expect("text"),
    )
}

fn sparql() -> QueryLanguage {
    QueryLanguage::try_from(SPARQL11_QUERY).expect("language")
}

#[tokio::test]
async fn streams_every_item() {
    let store = WordStore::new(false);
    let rows = store
        .query(
            request(support::store_iri(), sparql()),
            CancellationToken::new(),
        )
        .await
        .expect("started");
    let rows: Vec<_> = rows.collect().await;
    assert_eq!(
        rows,
        ["one", "two", "three"].map(|word| Ok(word.to_owned()))
    );
}

#[tokio::test]
async fn cancellation_ends_a_stalled_stream() {
    let store = WordStore::new(true);
    let cancellation = CancellationToken::new();
    let mut rows = store
        .query(
            request(support::store_iri(), sparql()),
            cancellation.clone(),
        )
        .await
        .expect("started");
    assert_eq!(rows.next().await, Some(Ok("one".to_owned())));
    cancellation.cancel();
    assert_eq!(rows.next().await, Some(Err(FacadeError::Cancelled)));
    assert_eq!(rows.next().await, None);
}

#[tokio::test]
async fn a_cancelled_request_never_executes() {
    let store = WordStore::new(false);
    let cancellation = CancellationToken::new();
    cancellation.cancel();
    let error = store
        .query(request(support::store_iri(), sparql()), cancellation)
        .await
        .err()
        .expect("cancelled");
    assert_eq!(error.code(), FacadeError::CODE_CANCELLED);
    assert_eq!(store.executed.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn unsupported_languages_and_other_stores_are_refused_before_execution() {
    let store = WordStore::new(false);
    let update = QueryLanguage::try_from(SPARQL11_UPDATE).expect("language");
    let error = store
        .query(
            request(support::store_iri(), update),
            CancellationToken::new(),
        )
        .await
        .err()
        .expect("no update");
    assert_eq!(error.code(), FacadeError::CODE_MISSING_CAPABILITIES);
    let other = StoreIri::try_new("https://example.org/other").expect("valid");
    let error = store
        .query(request(other, sparql()), CancellationToken::new())
        .await
        .err()
        .expect("other");
    assert_eq!(error.code(), FacadeError::CODE_STORE_MISMATCH);
    assert_eq!(store.executed.load(Ordering::SeqCst), 0);
}

#[test]
fn query_texts_must_not_be_blank() {
    assert_eq!(
        QueryText::try_new(" \n").expect_err("blank"),
        FacadeError::EmptyQuery
    );
}
