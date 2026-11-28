//! Streaming, back-pressure, cancellation and timeouts.

mod support;

use core::fmt::Write as _;
use core::num::NonZeroU16;
use core::time::Duration;
use std::io::Cursor;
use std::time::Instant;

use futures_util::StreamExt;
use store_facade_model::vocabulary::SPARQL11_QUERY;
use store_facade_model::{
    CancellableStream,
    FacadeError,
    QueryLanguage,
    QueryRequest,
    QueryStore,
    QueryText,
    QueryTimeout,
    StoreIri,
};
use store_facade_oxigraph::{
    OxigraphConfig,
    OxigraphStorage,
    OxigraphStore,
    QueryProgress,
    RdfFormat,
    ResultBuffer,
};
use tokio_util::sync::CancellationToken;

const BUFFER: u16 = 4;
const ROWS: u64 = 2_000;
const ALL_ROWS: &str = "SELECT ?s ?o WHERE { ?s <urn:p> ?o }";
/// Long enough for a lagging thread on a loaded CI runner.
const SETTLE: Duration = Duration::from_secs(10);

async fn seeded_store() -> OxigraphStore {
    let buffer = ResultBuffer::new(NonZeroU16::new(BUFFER).expect("non-zero"));
    let store =
        support::open(OxigraphConfig::new(OxigraphStorage::InMemory).with_result_buffer(buffer));
    let seed = (0..ROWS).fold(String::new(), |mut text, index| {
        let _ = writeln!(text, "<urn:s{index}> <urn:p> <urn:o{index}> .");
        text
    });
    store
        .load(RdfFormat::NQuads, Cursor::new(seed.into_bytes()))
        .await
        .expect("seeds");
    store
}

fn request(text: &str) -> QueryRequest {
    QueryRequest::new(
        StoreIri::try_new(support::STORE).expect("valid"),
        QueryLanguage::try_from(SPARQL11_QUERY).expect("a language"),
        QueryText::try_new(text).expect("non-empty"),
    )
}

async fn settled(progress: &QueryProgress) {
    let start = Instant::now();
    while !progress.is_finished() {
        assert!(
            start.elapsed() < SETTLE,
            "the evaluating thread never stopped"
        );
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}

/// Items in flight: those read, plus a full buffer, plus one held by a
/// blocked send.
const fn in_flight(read: u64) -> u64 {
    read + BUFFER as u64 + 1
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_slow_consumer_holds_the_evaluator_at_the_buffer() {
    let store = seeded_store().await;
    let mut stream = store
        .execute(request(ALL_ROWS), CancellationToken::new())
        .await
        .expect("starts");
    let progress = stream.progress();
    stream.next().await.expect("a row").expect("no error");
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(
        progress.items_produced() <= in_flight(1),
        "{} items produced ahead of one read",
        progress.items_produced()
    );
    assert!(!progress.is_finished());
    let mut read = 1;
    while let Some(item) = stream.next().await {
        item.expect("no error");
        read += 1;
        assert!(progress.items_produced() <= in_flight(read));
    }
    assert_eq!(read, ROWS);
    settled(&progress).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dropping_the_stream_stops_the_evaluator() {
    let store = seeded_store().await;
    let mut stream = store
        .execute(request(ALL_ROWS), CancellationToken::new())
        .await
        .expect("starts");
    let progress = stream.progress();
    stream.next().await.expect("a row").expect("no error");
    drop(stream);
    settled(&progress).await;
    assert!(progress.items_produced() <= in_flight(1));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancelling_mid_stream_ends_with_cancelled_and_stops_the_evaluator() {
    let store = seeded_store().await;
    let token = CancellationToken::new();
    let inner = store
        .execute(request(ALL_ROWS), token.clone())
        .await
        .expect("starts");
    let progress = inner.progress();
    let mut stream = CancellableStream::new(inner, token.clone());
    stream.next().await.expect("a row").expect("no error");
    token.cancel();
    assert_eq!(
        stream.next().await.expect("an item").err(),
        Some(FacadeError::Cancelled)
    );
    assert!(stream.next().await.is_none());
    settled(&progress).await;
    assert!(progress.items_produced() < ROWS);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_is_checked_between_rows_without_a_wrapper() {
    let store = seeded_store().await;
    let token = CancellationToken::new();
    let mut stream = store
        .execute(request(ALL_ROWS), token.clone())
        .await
        .expect("starts");
    let progress = stream.progress();
    stream.next().await.expect("a row").expect("no error");
    token.cancel();
    let mut last = None;
    while let Some(item) = stream.next().await {
        last = item.err();
    }
    assert_eq!(last, Some(FacadeError::Cancelled));
    settled(&progress).await;
    assert!(progress.items_produced() < ROWS);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_timeout_ends_a_slowly_read_stream() {
    let store = seeded_store().await;
    let timed = request(ALL_ROWS).with_timeout(QueryTimeout::try_from_millis(100).expect("valid"));
    let mut stream = store
        .query(timed, CancellationToken::new())
        .await
        .expect("starts");
    stream.next().await.expect("a row").expect("no error");
    tokio::time::sleep(Duration::from_millis(300)).await;
    let mut last = None;
    while let Some(item) = stream.next().await {
        last = item.err();
    }
    assert_eq!(last, Some(FacadeError::Timeout));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_cancelled_token_refuses_before_execution() {
    let store = seeded_store().await;
    let token = CancellationToken::new();
    token.cancel();
    assert_eq!(
        store.query(request(ALL_ROWS), token).await.err(),
        Some(FacadeError::Cancelled)
    );
}
