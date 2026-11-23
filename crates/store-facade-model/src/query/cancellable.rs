use core::pin::Pin;
use core::task::{
    Context,
    Poll,
};

use futures_core::Stream;
use tokio_util::sync::{
    CancellationToken,
    WaitForCancellationFutureOwned,
};

use crate::FacadeError;

/// A result stream that ends with [`FacadeError::Cancelled`] once its token
/// is cancelled, then yields nothing more.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CancellableStream<S> {
    inner: Option<S>,
    cancelled: Pin<Box<WaitForCancellationFutureOwned>>,
}

impl<S> CancellableStream<S> {
    /// Wraps `inner`, observing `cancellation`.
    pub fn new(inner: S, cancellation: CancellationToken) -> Self {
        Self {
            inner: Some(inner),
            cancelled: Box::pin(cancellation.cancelled_owned()),
        }
    }
}

impl<S, T> Stream for CancellableStream<S>
where
    S: Stream<Item = Result<T, FacadeError>> + Unpin,
{
    type Item = Result<T, FacadeError>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;
        if this.inner.is_none() {
            return Poll::Ready(None);
        }
        if this.cancelled.as_mut().poll(context).is_ready() {
            // Drop the inner stream at once so the backend releases the query.
            this.inner = None;
            return Poll::Ready(Some(Err(FacadeError::Cancelled)));
        }
        let Some(inner) = this.inner.as_mut() else {
            return Poll::Ready(None);
        };
        let next = Pin::new(inner).poll_next(context);
        if matches!(next, Poll::Ready(None)) {
            this.inner = None;
        }
        next
    }
}
