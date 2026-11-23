use core::future::Future;

use futures_core::Stream;
use tokio_util::sync::CancellationToken;

use crate::{
    CancellableStream,
    FacadeError,
    QueryRequest,
    StoreDescription,
};

/// A store that answers queries as a stream.
///
/// Implementors supply [`QueryStore::description`] and
/// [`QueryStore::execute`]. Callers use [`QueryStore::query`], which checks
/// the request against the description before execution and ends the stream
/// with [`FacadeError::Cancelled`] as soon as the token is cancelled.
/// Dropping the stream releases the query.
pub trait QueryStore: Send + Sync {
    /// One streamed result item, such as a solution or a result frame.
    type Item: Send;
    /// The result stream.
    type Stream: Stream<Item = Result<Self::Item, FacadeError>> + Send + Unpin;

    /// The store's description.
    fn description(&self) -> &StoreDescription;

    /// Starts a checked request.
    ///
    /// Implementations should observe `cancellation` while starting; the
    /// returned stream is wrapped by [`QueryStore::query`].
    fn execute(
        &self,
        request: QueryRequest,
        cancellation: CancellationToken,
    ) -> impl Future<Output = Result<Self::Stream, FacadeError>> + Send;

    /// Checks and starts `request`, returning a cancellable stream.
    ///
    /// # Errors
    ///
    /// [`FacadeError::StoreMismatch`], [`FacadeError::MissingCapabilities`],
    /// [`FacadeError::Cancelled`] when cancelled before the stream starts, or
    /// any error of [`QueryStore::execute`].
    fn query(
        &self,
        request: QueryRequest,
        cancellation: CancellationToken,
    ) -> impl Future<Output = Result<CancellableStream<Self::Stream>, FacadeError>> + Send {
        async move {
            request.check_against(self.description())?;
            if cancellation.is_cancelled() {
                return Err(FacadeError::Cancelled);
            }
            let stream = self.execute(request, cancellation.clone()).await?;
            Ok(CancellableStream::new(stream, cancellation))
        }
    }
}
