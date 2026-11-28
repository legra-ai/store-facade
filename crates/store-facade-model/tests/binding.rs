//! Operator-side bindings and binders.

mod support;

use futures_util::stream::{
    self,
    BoxStream,
};
use serde::{
    Deserialize,
    Serialize,
};
use store_facade_model::vocabulary::SPARQL11_QUERY;
use store_facade_model::{
    Binder,
    CapabilitySet,
    FacadeError,
    ImplementationName,
    QueryRequest,
    QueryStore,
    StoreBinding,
    StoreDescription,
    StoreLabel,
    StoreLabels,
    StoreSlug,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EchoConfig {
    rows: u8,
}

struct EchoStore {
    description: StoreDescription,
}

impl QueryStore for EchoStore {
    type Item = u8;
    type Stream = BoxStream<'static, Result<u8, FacadeError>>;

    fn description(&self) -> &StoreDescription {
        &self.description
    }

    fn execute(
        &self,
        _request: QueryRequest,
        _cancellation: CancellationToken,
    ) -> impl Future<Output = Result<Self::Stream, FacadeError>> + Send {
        core::future::ready(Ok(Box::pin(stream::empty()) as Self::Stream))
    }
}

struct EchoBinder;

impl Binder for EchoBinder {
    type Config = EchoConfig;
    type Store = EchoStore;

    fn implementation(&self) -> ImplementationName {
        ImplementationName::try_new("echo").expect("valid name")
    }

    fn bind(&self, binding: StoreBinding<EchoConfig>) -> Result<EchoStore, FacadeError> {
        self.check(&binding)?;
        let (description, _config) =
            binding.into_parts(CapabilitySet::single(SPARQL11_QUERY.capability()))?;
        Ok(EchoStore { description })
    }
}

fn labels() -> StoreLabels {
    let mut labels = StoreLabels::new();
    labels
        .insert(StoreLabel::new(
            support::tag("en"),
            store_facade_model::LabelText::try_new("Sales").expect("valid label"),
        ))
        .expect("one label");
    labels
}

fn binding(implementation: &str) -> StoreBinding<EchoConfig> {
    StoreBinding::try_new(
        support::store_iri(),
        StoreSlug::try_new("sales").expect("valid slug"),
        labels(),
        ImplementationName::try_new(implementation).expect("valid name"),
        EchoConfig { rows: 3 },
    )
    .expect("valid binding")
}

#[test]
fn a_binder_opens_its_own_bindings_with_a_neutral_description() {
    let store = EchoBinder.bind(binding("echo")).expect("binds");
    let description = store.description();
    assert_eq!(description.iri(), &support::store_iri());
    assert!(
        description
            .capabilities()
            .contains(&SPARQL11_QUERY.capability())
    );
    let rendered = format!("{description:?}");
    assert!(!rendered.contains("echo"), "{rendered}");
}

#[test]
fn a_binder_refuses_another_implementation() {
    let Err(error) = EchoBinder.bind(binding("other")) else {
        panic!("expected a mismatch");
    };
    assert_eq!(error.code(), FacadeError::CODE_BINDING_MISMATCH);
}

#[test]
fn implementation_names_follow_the_slug_rules() {
    for bad in ["", "Oxigraph", "a--b", "b-", "1st"] {
        assert_eq!(
            ImplementationName::try_new(bad)
                .expect_err("invalid")
                .code(),
            FacadeError::CODE_INVALID_IMPLEMENTATION_NAME,
            "{bad:?}"
        );
    }
}

#[test]
fn a_binding_needs_a_label_and_round_trips() {
    assert_eq!(
        StoreBinding::try_new(
            support::store_iri(),
            StoreSlug::try_new("sales").expect("valid slug"),
            StoreLabels::new(),
            ImplementationName::try_new("echo").expect("valid name"),
            EchoConfig { rows: 1 },
        )
        .expect_err("no label"),
        FacadeError::MissingLabel
    );
    let binding = binding("echo");
    assert_eq!(binding.config(), &EchoConfig { rows: 3 });
    assert_eq!(binding.implementation().as_str(), "echo");
    let json = serde_json::to_string(&binding).expect("serializes");
    assert_eq!(
        serde_json::from_str::<StoreBinding<EchoConfig>>(&json).expect("parses"),
        binding
    );
}
