use core::fmt;
use core::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    FacadeError,
    QueryRequest,
};

/// A BLAKE3 digest identifying a cacheable response by store, language and
/// query text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CacheKey([u8; 32]);

impl CacheKey {
    /// Separates this preimage from every other BLAKE3 use; bump on change.
    const DOMAIN: &'static str = "store-facade/cache-key/v1";

    /// The key of `request`.
    ///
    /// The preimage is the domain followed by each field, length-prefixed as
    /// a little-endian `u64`, so no two requests share a preimage.
    #[must_use]
    pub fn for_request(request: &QueryRequest) -> Self {
        let mut hasher = blake3::Hasher::new();
        for field in [
            Self::DOMAIN,
            request.store().as_str(),
            request.language().capability().as_str(),
            request.text().as_str(),
        ] {
            hasher.update(&(field.len() as u64).to_le_bytes());
            hasher.update(field.as_bytes());
        }
        Self(*hasher.finalize().as_bytes())
    }

    /// The digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for CacheKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|byte| write!(formatter, "{byte:02x}"))
    }
}

impl FromStr for CacheKey {
    type Err = FacadeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let digits = text.as_bytes();
        if digits.len() != 64 {
            return Err(FacadeError::InvalidCacheKey);
        }
        let mut bytes = [0_u8; 32];
        let (pairs, _) = digits.as_chunks::<2>();
        for (byte, pair) in bytes.iter_mut().zip(pairs) {
            *byte = (Self::nibble(pair[0])? << 4) | Self::nibble(pair[1])?;
        }
        Ok(Self(bytes))
    }
}

impl CacheKey {
    fn nibble(digit: u8) -> Result<u8, FacadeError> {
        match digit {
            b'0'..=b'9' => Ok(digit - b'0'),
            b'a'..=b'f' => Ok(digit - b'a' + 10),
            _ => Err(FacadeError::InvalidCacheKey),
        }
    }
}

impl TryFrom<String> for CacheKey {
    type Error = FacadeError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        text.parse()
    }
}

impl From<CacheKey> for String {
    fn from(key: CacheKey) -> Self {
        key.to_string()
    }
}
