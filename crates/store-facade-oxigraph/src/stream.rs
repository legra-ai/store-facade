use core::pin::Pin;
use core::task::{
    Context,
    Poll,
};

use futures_core::Stream;
use store_facade_model::FacadeError;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::{
    CancellationToken,
    DropGuard,
    WaitForCancellationFutureOwned,
};

use crate::{
    QueryProgress,
    ResultItem,
};

/// The results of one query, read from a bounded channel its evaluating
/// thread fills.
///
/// It ends with [`FacadeError::Timeout`] as soon as the request's timeout
/// elapses. Dropping the stream stops the evaluation.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct ResultStream {
    receiver: Option<Receiver<Result<ResultItem, FacadeError>>>,
    progress: QueryProgress,
    deadline: Option<Pin<Box<WaitForCancellationFutureOwned>>>,
    _release: Option<DropGuard>,
}

impl ResultStream {
    pub(crate) fn new(
        receiver: Receiver<Result<ResultItem, FacadeError>>,
        progress: QueryProgress,
        timed_out: CancellationToken,
        release: DropGuard,
    ) -> Self {
        Self {
            receiver: Some(receiver),
            progress,
            deadline: Some(Box::pin(timed_out.cancelled_owned())),
            _release: Some(release),
        }
    }

    /// The stream of an update, which yields no items.
    pub(crate) fn empty() -> Self {
        Self {
            receiver: None,
            progress: QueryProgress::finished(),
            deadline: None,
            _release: None,
        }
    }

    /// The progress of the evaluating thread.
    #[must_use]
    pub fn progress(&self) -> QueryProgress {
        self.progress.clone()
    }
}

impl Stream for ResultStream {
    type Item = Result<ResultItem, FacadeError>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        if this.receiver.is_none() {
            return Poll::Ready(None);
        }
        if let Some(deadline) = this.deadline.as_mut()
            && deadline.as_mut().poll(context).is_ready()
        {
            // Release the channel at once; the evaluator stops at its next
            // checkpoint or send.
            this.receiver = None;
            this.deadline = None;
            return Poll::Ready(Some(Err(FacadeError::Timeout)));
        }
        let Some(receiver) = this.receiver.as_mut() else {
            return Poll::Ready(None);
        };
        let next = receiver.poll_recv(context);
        if matches!(next, Poll::Ready(None)) {
            this.receiver = None;
        }
        next
    }
}
