use core::fmt;

use serde::Serialize;

/// A stable, machine-readable error code.
///
/// Codes are part of the public contract: a code, once published, keeps its
/// spelling and meaning. The catalogue is [`crate::FacadeError::CODES`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    /// Declares a code; only the catalogue in [`crate::FacadeError`] uses it.
    pub(crate) const fn new(code: &'static str) -> Self {
        Self(code)
    }

    /// The code's stable spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
