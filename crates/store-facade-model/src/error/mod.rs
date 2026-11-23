//! The error type and its stable code catalogue.

mod code;
mod facade_error;
mod slug_rejection;

pub use code::ErrorCode;
pub use facade_error::FacadeError;
pub use slug_rejection::SlugRejection;
