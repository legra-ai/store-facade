use std::sync::Arc;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    Ordering,
};

/// A view of one query's evaluating thread: how many items it handed over
/// and whether it has stopped.
///
/// The item count stays within the configured [`crate::ResultBuffer`] of the
/// items the consumer has read.
#[derive(Debug, Clone, Default)]
pub struct QueryProgress(Arc<ProgressState>);

#[derive(Debug, Default)]
struct ProgressState {
    items: AtomicU64,
    finished: AtomicBool,
}

impl QueryProgress {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// A progress that has already finished, for streams without a thread.
    pub(crate) fn finished() -> Self {
        let progress = Self::new();
        progress.finish();
        progress
    }

    /// Items handed to the result channel so far, read or not.
    #[must_use]
    pub fn items_produced(&self) -> u64 {
        self.0.items.load(Ordering::SeqCst)
    }

    /// Whether the evaluating thread has stopped.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.0.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn record_item(&self) {
        self.0.items.fetch_add(1, Ordering::SeqCst);
    }

    pub(crate) fn finish(&self) {
        self.0.finished.store(true, Ordering::SeqCst);
    }
}
