use core::iter;

use iri_rfc3987::Iri;
use rdf_structural_model::{
    Literal,
    Subject,
    Term,
    Triple,
};

use crate::StoreDescription;
use crate::vocabulary::{
    RDF_TYPE,
    RDFS_LABEL,
    SF_SLUG,
    SF_STORE,
};

impl StoreDescription {
    /// The description as RDF triples, produced lazily one at a time.
    ///
    /// The order is fixed: `rdf:type sf:Store`, `sf:slug`, the `rdfs:label`s
    /// by language tag, then the capabilities by role and IRI.
    /// [`crate::StoreDescriptionReader`] reads them back in any order.
    pub fn triples(&self) -> impl Iterator<Item = Triple> + '_ {
        let store = self.iri().as_iri();
        let statement = move |predicate: &'static str, object: Term| {
            Triple::new(Subject::Iri(store.clone()), Iri::known(predicate), object)
        };
        iter::once(statement(RDF_TYPE, Term::iri(Iri::known(SF_STORE))))
            .chain(iter::once(statement(
                SF_SLUG,
                Term::literal(Literal::string(self.slug().as_str())),
            )))
            .chain(
                self.labels()
                    .iter()
                    .map(move |label| statement(RDFS_LABEL, Term::literal(label.to_literal()))),
            )
            .chain(self.capabilities().iter().map(move |capability| {
                statement(
                    capability.role().predicate(),
                    Term::iri(capability.iri().clone()),
                )
            }))
    }
}
