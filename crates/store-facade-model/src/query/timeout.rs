use core::fmt;
use core::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

use crate::FacadeError;

/// How long a caller lets one query run, from its start to its last result.
///
/// A timeout is 1 millisecond to 24 hours, at millisecond precision, and
/// serializes as its whole number of milliseconds. A request that carries one
/// requires [`crate::vocabulary::PER_QUERY_TIMEOUT`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct QueryTimeout(u64);

impl QueryTimeout {
    /// The longest accepted timeout, in milliseconds: 24 hours.
    pub const MAX_MILLIS: u64 = 24 * 60 * 60 * 1000;

    /// Checks and wraps a timeout of `millis` milliseconds.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidQueryTimeout`] for zero or more than
    /// [`QueryTimeout::MAX_MILLIS`].
    pub const fn try_from_millis(millis: u64) -> Result<Self, FacadeError> {
        if millis == 0 {
            Err(FacadeError::InvalidQueryTimeout {
                reason: "it must be at least 1 millisecond",
            })
        } else if millis > Self::MAX_MILLIS {
            Err(FacadeError::InvalidQueryTimeout {
                reason: "it must not exceed 24 hours",
            })
        } else {
            Ok(Self(millis))
        }
    }

    /// Checks and wraps `duration`, truncated to whole milliseconds.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidQueryTimeout`] as for
    /// [`QueryTimeout::try_from_millis`].
    pub fn try_from_duration(duration: Duration) -> Result<Self, FacadeError> {
        let millis = u64::try_from(duration.as_millis()).unwrap_or(u64::MAX);
        Self::try_from_millis(millis)
    }

    /// The timeout in milliseconds.
    #[must_use]
    pub const fn as_millis(self) -> u64 {
        self.0
    }

    /// The timeout as a duration.
    #[must_use]
    pub const fn as_duration(self) -> Duration {
        Duration::from_millis(self.0)
    }
}

impl TryFrom<u64> for QueryTimeout {
    type Error = FacadeError;

    fn try_from(millis: u64) -> Result<Self, Self::Error> {
        Self::try_from_millis(millis)
    }
}

impl From<QueryTimeout> for u64 {
    fn from(timeout: QueryTimeout) -> Self {
        timeout.0
    }
}

impl fmt::Display for QueryTimeout {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ms", self.0)
    }
}
