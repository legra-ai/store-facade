use core::future;

use oxigraph::sparql::CancellationToken as EvaluationToken;
use store_facade_model::{
    FacadeError,
    QueryTimeout,
};
use tokio_util::sync::{
    CancellationToken,
    DropGuard,
};

/// Why and whether one execution must stop.
///
/// A watcher task stops the evaluation (through its evaluation token) when
/// the caller cancels, when the timeout elapses, or when the execution is
/// released; the evaluating thread reads the reason back as the error.
#[derive(Clone)]
pub(crate) struct Interruption {
    caller: CancellationToken,
    evaluation: EvaluationToken,
    timed_out: CancellationToken,
    released: CancellationToken,
}

impl Interruption {
    /// Starts watching `caller` and `timeout`; must run inside a runtime.
    pub(crate) fn start(caller: CancellationToken, timeout: Option<QueryTimeout>) -> Self {
        let interruption = Self {
            caller,
            evaluation: EvaluationToken::new(),
            timed_out: CancellationToken::new(),
            released: CancellationToken::new(),
        };
        tokio::spawn(interruption.clone().watch(timeout));
        interruption
    }

    async fn watch(self, timeout: Option<QueryTimeout>) {
        let elapsed = async {
            match timeout {
                Some(timeout) => tokio::time::sleep(timeout.as_duration()).await,
                None => future::pending().await,
            }
        };
        tokio::select! {
            () = self.caller.cancelled() => {}
            () = self.released.cancelled() => {}
            () = elapsed => self.timed_out.cancel(),
        }
        self.evaluation.cancel();
    }

    /// The token the evaluator polls.
    pub(crate) fn evaluation_token(&self) -> EvaluationToken {
        self.evaluation.clone()
    }

    /// Whether the execution must stop now.
    pub(crate) fn is_interrupted(&self) -> bool {
        self.caller.is_cancelled() || self.evaluation.is_cancelled()
    }

    /// The error that reports the interruption: [`FacadeError::Timeout`] once
    /// the timeout elapsed, else [`FacadeError::Cancelled`].
    pub(crate) fn error(&self) -> FacadeError {
        if self.timed_out.is_cancelled() {
            FacadeError::Timeout
        } else {
            FacadeError::Cancelled
        }
    }

    /// A token cancelled once the timeout elapses, which the result stream
    /// observes so the caller's bound holds even while the evaluator has
    /// yet to reach its next checkpoint.
    pub(crate) fn timed_out(&self) -> CancellationToken {
        self.timed_out.clone()
    }

    /// A guard that releases the execution, stopping its evaluation and its
    /// watcher, when dropped.
    pub(crate) fn release_on_drop(&self) -> DropGuard {
        self.released.clone().drop_guard()
    }
}
