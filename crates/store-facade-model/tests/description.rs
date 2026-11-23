//! Building store descriptions.

mod support;

use store_facade_model::vocabulary::{
    NAMED_GRAPHS,
    SPARQL11_QUERY,
    TRANSACTIONS,
};
use store_facade_model::{
    CapabilitySet,
    DescriptionLimit,
    FacadeError,
    LabelText,
    RequiredCapabilities,
    StoreDescription,
    StoreLabel,
    StoreLabels,
    StoreSlug,
};
use support::{
    sample,
    store_iri,
    tag,
};

fn slug() -> StoreSlug {
    StoreSlug::try_new("sales").expect("valid")
}

#[test]
fn builder_and_try_new_agree() {
    let built = sample();
    let mut labels = StoreLabels::new();
    for label in built.labels().iter() {
        labels.insert(label).expect("distinct languages");
    }
    let assembled =
        StoreDescription::try_new(store_iri(), slug(), labels, built.capabilities().clone())
            .expect("valid");
    assert_eq!(assembled, built);
    assert_eq!(
        built
            .labels()
            .get(&tag("EN"))
            .expect("case-insensitive")
            .as_str(),
        "Sales ledger"
    );
}

#[test]
fn a_description_needs_a_label() {
    let error = StoreDescription::builder(store_iri(), slug())
        .build()
        .expect_err("no label");
    assert_eq!(error, FacadeError::MissingLabel);
    let error = StoreDescription::try_new(
        store_iri(),
        slug(),
        StoreLabels::new(),
        CapabilitySet::new(),
    )
    .expect_err("no label");
    assert_eq!(error.code(), FacadeError::CODE_MISSING_LABEL);
}

#[test]
fn one_label_per_language() {
    let restated = StoreDescription::builder(store_iri(), slug())
        .label(tag("en"), "Sales")
        .label(tag("en"), "Sales")
        .build()
        .expect("an identical restatement is accepted");
    assert_eq!(restated.labels().len(), 1);
    let error = StoreDescription::builder(store_iri(), slug())
        .label(tag("en"), "Sales")
        .label(tag("en-GB"), "Sales")
        .label(tag("en"), "Revenue")
        .build()
        .expect_err("conflicting");
    assert_eq!(error.code(), FacadeError::CODE_INVALID_LABEL);
}

#[test]
fn label_text_is_checked_and_bounded() {
    assert_eq!(
        LabelText::try_new("  ").expect_err("blank").code(),
        FacadeError::CODE_INVALID_LABEL
    );
    let limit = DescriptionLimit::LabelBytes.max();
    LabelText::try_new("a".repeat(limit)).expect("at the limit");
    assert_eq!(
        LabelText::try_new("a".repeat(limit + 1)).expect_err("too long"),
        FacadeError::LimitExceeded {
            limit: DescriptionLimit::LabelBytes
        }
    );
}

#[test]
fn labels_are_bounded() {
    let mut labels = StoreLabels::new();
    let tags = (0..=DescriptionLimit::Labels.max()).map(|index| tag(&format!("x-l{index}")));
    let results: Vec<_> = tags
        .map(|language| {
            labels.insert(StoreLabel::new(
                language,
                LabelText::try_new("label").expect("valid"),
            ))
        })
        .collect();
    assert!(
        results[..DescriptionLimit::Labels.max()]
            .iter()
            .all(Result::is_ok)
    );
    assert_eq!(
        results
            .last()
            .expect("one too many")
            .clone()
            .expect_err("over"),
        FacadeError::LimitExceeded {
            limit: DescriptionLimit::Labels
        }
    );
}

#[test]
fn the_first_builder_error_wins() {
    let error = StoreDescription::builder(store_iri(), slug())
        .label(tag("en"), "")
        .label(tag("nl"), "Verkoop")
        .capability(SPARQL11_QUERY)
        .build()
        .expect_err("blank label");
    assert_eq!(error.code(), FacadeError::CODE_INVALID_LABEL);
}

#[test]
fn descriptions_check_requirements() {
    let description = sample();
    description
        .require(
            &RequiredCapabilities::single(SPARQL11_QUERY)
                .with(NAMED_GRAPHS)
                .expect("small"),
        )
        .expect("present");
    let error = description
        .require(&RequiredCapabilities::single(TRANSACTIONS))
        .expect_err("absent");
    assert_eq!(error.code(), FacadeError::CODE_MISSING_CAPABILITIES);
}
