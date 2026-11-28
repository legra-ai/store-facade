//! A contract binding over the Oxigraph binder, plus shared fixtures.
#![allow(dead_code)]

use std::io::Cursor;

use rdf_structural_model::LanguageTag;
use store_facade_contract::{
    ContractBinding,
    Observation,
    SeedQuads,
    SolutionObservation,
    TermText,
    VariableName,
};
use store_facade_model::{
    Binder,
    FacadeError,
    ImplementationName,
    LabelText,
    StoreBinding,
    StoreIri,
    StoreLabel,
    StoreLabels,
    StoreSlug,
};
use store_facade_oxigraph::{
    OxigraphBinder,
    OxigraphConfig,
    OxigraphStorage,
    OxigraphStore,
    RdfFormat,
    ResultItem,
};
use tempfile::TempDir;

pub const STORE: &str = "https://example.org/store/sales";

/// One fresh store per test, in memory or in its own temporary directory.
pub struct OxigraphContract {
    config: OxigraphConfig,
    _directory: Option<TempDir>,
}

impl OxigraphContract {
    pub fn memory() -> Self {
        Self {
            config: OxigraphConfig::new(OxigraphStorage::InMemory),
            _directory: None,
        }
    }

    pub fn disk() -> Self {
        let directory = TempDir::new().expect("a temporary directory");
        Self {
            config: OxigraphConfig::new(OxigraphStorage::on_disk(directory.path())),
            _directory: Some(directory),
        }
    }

    pub fn with_union_default_graph(mut self) -> Self {
        self.config = self.config.with_union_default_graph();
        self
    }
}

impl ContractBinding for OxigraphContract {
    type Store = OxigraphStore;

    fn open(&self) -> impl Future<Output = Result<OxigraphStore, FacadeError>> + Send {
        core::future::ready(OxigraphBinder.bind(binding(self.config.clone())))
    }

    async fn seed(&self, store: &OxigraphStore, quads: &SeedQuads) -> Result<(), FacadeError> {
        let bytes = quads.as_bytes().to_vec();
        store.load(RdfFormat::NQuads, Cursor::new(bytes)).await
    }

    fn observe(item: &ResultItem) -> Observation {
        match item {
            ResultItem::Boolean(answer) => Observation::Boolean(*answer),
            ResultItem::Solution(solution) => Observation::Solution(solution.iter().fold(
                SolutionObservation::new(),
                |observed, (variable, term)| {
                    observed.bind(
                        VariableName::new(variable.as_str()),
                        TermText::new(term.to_string()),
                    )
                },
            )),
            ResultItem::Triple(triple) => Observation::Triple(
                TermText::new(triple.subject.to_string()),
                TermText::new(triple.predicate.to_string()),
                TermText::new(triple.object.to_string()),
            ),
        }
    }
}

pub fn labels() -> StoreLabels {
    let mut labels = StoreLabels::new();
    labels
        .insert(StoreLabel::new(
            rdf_tag("en"),
            LabelText::try_new("Sales").expect("a valid label"),
        ))
        .expect("one label");
    labels
}

fn rdf_tag(text: &str) -> LanguageTag {
    LanguageTag::try_new(text.to_owned()).expect("a valid language tag")
}

pub fn binding(config: OxigraphConfig) -> StoreBinding<OxigraphConfig> {
    StoreBinding::try_new(
        StoreIri::try_new(STORE).expect("a valid Store IRI"),
        StoreSlug::try_new("sales").expect("a valid slug"),
        labels(),
        ImplementationName::try_new(OxigraphBinder::IMPLEMENTATION).expect("a valid name"),
        config,
    )
    .expect("a valid binding")
}

/// An in-memory store with the given configuration.
pub fn open(config: OxigraphConfig) -> OxigraphStore {
    OxigraphBinder
        .bind(binding(config))
        .expect("the store opens")
}
