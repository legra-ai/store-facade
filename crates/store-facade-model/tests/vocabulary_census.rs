//! Every vocabulary constant is defined in the vendored Turtle file, and
//! every term the file defines is a constant.

use std::collections::BTreeSet;

use store_facade_model::CapabilityRole;
use store_facade_model::vocabulary::{
    ALL_CAPABILITIES,
    SF,
    SF_SLUG,
    SF_STORE,
    TURTLE,
};

/// Prefix declarations of the vendored file.
fn prefixes() -> Vec<(String, String)> {
    TURTLE
        .lines()
        .filter_map(|line| line.strip_prefix("@prefix "))
        .map(|rest| {
            let (prefix, namespace) = rest.split_once(':').expect("prefix:");
            let namespace = namespace.trim().trim_start_matches('<');
            let namespace = namespace.split_once('>').expect("<namespace>").0;
            (prefix.trim().to_owned(), namespace.to_owned())
        })
        .collect()
}

fn expand(name: &str, prefixes: &[(String, String)]) -> String {
    let (prefix, local) = name.split_once(':').expect("prefixed name");
    let namespace = prefixes
        .iter()
        .find(|(p, _)| p == prefix)
        .expect("declared prefix");
    format!("{}{local}", namespace.1)
}

/// `(subject, class)` for every subject line `<subject> a <class> ;`.
fn definitions() -> BTreeSet<(String, String)> {
    let prefixes = prefixes();
    TURTLE
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with([' ', '#', '@']))
        .map(|line| {
            let mut words = line.split_whitespace();
            let subject = words.next().expect("subject");
            assert_eq!(
                words.next(),
                Some("a"),
                "subject lines start with a type: {line}"
            );
            let class = words.next().expect("class");
            (expand(subject, &prefixes), expand(class, &prefixes))
        })
        .collect()
}

#[test]
fn constants_and_turtle_define_the_same_capabilities() {
    let in_file: BTreeSet<_> = definitions()
        .into_iter()
        .filter(|(_, class)| CapabilityRole::ALL.iter().any(|role| role.class() == class))
        .collect();
    let in_code: BTreeSet<_> = ALL_CAPABILITIES
        .iter()
        .map(|term| (term.iri().to_owned(), term.role().class().to_owned()))
        .collect();
    assert_eq!(in_file, in_code);
    assert_eq!(
        in_code.len(),
        ALL_CAPABILITIES.len(),
        "no duplicate constants"
    );
}

#[test]
fn every_store_facade_term_is_a_constant() {
    let defined: BTreeSet<String> = definitions()
        .into_iter()
        .map(|(subject, _)| subject)
        .filter(|subject| subject.starts_with(SF))
        .collect();
    let constants: BTreeSet<String> = ALL_CAPABILITIES
        .iter()
        .map(|term| term.iri())
        .chain([SF_STORE, SF_SLUG])
        .filter(|iri| iri.starts_with(SF))
        .map(str::to_owned)
        .collect();
    assert_eq!(defined, constants);
}

#[test]
fn description_terms_are_defined() {
    let definitions = definitions();
    let rdfs_class = "http://www.w3.org/2000/01/rdf-schema#Class";
    let rdf_property = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
    assert!(definitions.contains(&(SF_STORE.to_owned(), rdfs_class.to_owned())));
    assert!(definitions.contains(&(SF_SLUG.to_owned(), rdf_property.to_owned())));
}
