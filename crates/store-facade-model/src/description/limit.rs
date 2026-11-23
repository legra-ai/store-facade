use core::fmt;

/// The bounds that keep a store description small.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptionLimit {
    /// At most 256 capabilities in one set.
    Capabilities,
    /// At most 64 labels, one per language tag.
    Labels,
    /// At most 1,024 bytes of label text.
    LabelBytes,
}

impl DescriptionLimit {
    /// The limit's value.
    #[must_use]
    pub const fn max(self) -> usize {
        match self {
            Self::Capabilities => 256,
            Self::Labels => 64,
            Self::LabelBytes => 1024,
        }
    }
}

impl fmt::Display for DescriptionLimit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let what = match self {
            Self::Capabilities => "capabilities per set",
            Self::Labels => "labels per store",
            Self::LabelBytes => "bytes per label",
        };
        write!(formatter, "the limit of {} {what}", self.max())
    }
}
