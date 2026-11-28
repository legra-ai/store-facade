/// N-Quads text (RDF 1.2 where a check needs it) that seeds a store.
///
/// Every seed is a small constant or generated text of the contract suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedQuads(String);

impl SeedQuads {
    pub(crate) fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// The N-Quads text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The N-Quads text as bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
