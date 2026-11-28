use core::num::NonZeroU16;
use std::path::{
    Path,
    PathBuf,
};

use oxigraph::store::Store;
use serde::{
    Deserialize,
    Serialize,
};
use store_facade_model::vocabulary::{
    CapabilityTerm,
    NAMED_GRAPHS,
    PER_QUERY_TIMEOUT,
    RDF12_TERMS,
    SPARQL11_QUERY,
    SPARQL11_UPDATE,
    SPARQL12_QUERY,
    SPARQL12_UPDATE,
    TRANSACTIONS,
    UNION_DEFAULT_GRAPH,
};
use store_facade_model::{
    CapabilitySet,
    FacadeError,
};

use crate::error::ErrorTranslation;

/// Where an Oxigraph store keeps its data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum OxigraphStorage {
    /// In process memory; the data ends with the store.
    InMemory,
    /// In a directory on disk, created when absent.
    OnDisk {
        /// The store directory.
        path: PathBuf,
    },
}

impl OxigraphStorage {
    /// On-disk storage in `path`.
    #[must_use]
    pub fn on_disk(path: impl AsRef<Path>) -> Self {
        Self::OnDisk {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub(crate) fn open(&self) -> Result<Store, FacadeError> {
        match self {
            Self::InMemory => Store::new(),
            Self::OnDisk { path } => Store::open(path),
        }
        .map_err(ErrorTranslation::from_storage)
    }
}

/// How many result items a query may buffer ahead of its consumer.
///
/// The evaluating thread blocks once the buffer is full, so a slow consumer
/// holds memory at this bound however large the result is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResultBuffer(NonZeroU16);

impl ResultBuffer {
    /// The default: 64 items.
    pub const DEFAULT: Self = Self(NonZeroU16::new(64).expect("non-zero"));

    /// A buffer of `items` items.
    #[must_use]
    pub const fn new(items: NonZeroU16) -> Self {
        Self(items)
    }

    /// The capacity in items.
    #[must_use]
    pub const fn items(self) -> usize {
        self.0.get() as usize
    }
}

impl Default for ResultBuffer {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The operator-side configuration of an Oxigraph store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OxigraphConfig {
    storage: OxigraphStorage,
    #[serde(default)]
    union_default_graph: bool,
    #[serde(default)]
    result_buffer: ResultBuffer,
}

impl OxigraphConfig {
    /// A store in `storage`, with a separate default graph and the default
    /// result buffer.
    #[must_use]
    pub const fn new(storage: OxigraphStorage) -> Self {
        Self {
            storage,
            union_default_graph: false,
            result_buffer: ResultBuffer::DEFAULT,
        }
    }

    /// Queries see the union of all named graphs as their default graph.
    #[must_use]
    pub const fn with_union_default_graph(mut self) -> Self {
        self.union_default_graph = true;
        self
    }

    /// Buffers at most `buffer` items per query.
    #[must_use]
    pub const fn with_result_buffer(mut self, buffer: ResultBuffer) -> Self {
        self.result_buffer = buffer;
        self
    }

    /// The storage.
    #[must_use]
    pub const fn storage(&self) -> &OxigraphStorage {
        &self.storage
    }

    /// Whether the default graph is the union of the named graphs.
    #[must_use]
    pub const fn union_default_graph(&self) -> bool {
        self.union_default_graph
    }

    /// The per-query result buffer.
    #[must_use]
    pub const fn result_buffer(&self) -> ResultBuffer {
        self.result_buffer
    }

    /// The capabilities a store of this configuration supports.
    ///
    /// Exact literal preservation is not among them: the store keeps typed
    /// literals in canonical form, so `"01"^^xsd:integer` returns as `"1"`.
    #[must_use]
    pub fn capabilities(&self) -> CapabilitySet {
        let union = self
            .union_default_graph
            .then(|| UNION_DEFAULT_GRAPH.capability());
        // bounded: nine catalogue terms at most.
        CapabilitySet::try_from_iter(
            [
                SPARQL11_QUERY,
                SPARQL11_UPDATE,
                SPARQL12_QUERY,
                SPARQL12_UPDATE,
                RDF12_TERMS,
                NAMED_GRAPHS,
                TRANSACTIONS,
                PER_QUERY_TIMEOUT,
            ]
            .map(CapabilityTerm::capability)
            .into_iter()
            .chain(union),
        )
        .unwrap_or_default()
    }
}
