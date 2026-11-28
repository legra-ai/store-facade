//! The positive check of each capability, run when a store declares it.

mod entailment;
mod geosparql;
mod graphs;
mod literals;
mod query;
mod rdf12;
mod timeout;
mod transactions;
mod unverifiable;
mod update;

pub(crate) use entailment::{
    owl_rdf_based,
    rdfs,
};
pub(crate) use geosparql::geosparql;
pub(crate) use graphs::{
    named_graphs,
    union_default_graph,
};
pub(crate) use literals::exact_literal_preservation;
pub(crate) use query::{
    sparql11_query,
    sparql12_query,
};
pub(crate) use rdf12::rdf12_terms;
pub(crate) use timeout::per_query_timeout;
pub(crate) use transactions::transactions;
pub(crate) use unverifiable::unverifiable;
pub(crate) use update::{
    sparql11_update,
    sparql12_update,
};
