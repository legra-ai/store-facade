use core::fmt;

/// Why a candidate [`crate::StoreSlug`] was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlugRejection {
    /// The slug is empty.
    Empty,
    /// The slug exceeds [`crate::StoreSlug::MAX_LEN`] bytes.
    TooLong,
    /// The slug does not start with a lowercase ASCII letter.
    InvalidStart,
    /// The slug contains a byte other than `a-z`, `0-9` or `-`.
    InvalidCharacter,
    /// The slug ends with `-` or contains `--`.
    InvalidHyphen,
}

impl fmt::Display for SlugRejection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "it is empty",
            Self::TooLong => "it exceeds 63 bytes",
            Self::InvalidStart => "it must start with a lowercase ASCII letter",
            Self::InvalidCharacter => "only a-z, 0-9 and '-' are allowed",
            Self::InvalidHyphen => "it must not end with '-' or contain '--'",
        })
    }
}
