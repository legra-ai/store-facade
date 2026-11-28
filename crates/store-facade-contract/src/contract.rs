use store_facade_model::vocabulary::{
    ALL_CAPABILITIES,
    CapabilityTerm,
    ENTAILMENT_OWL_RDF_BASED,
    ENTAILMENT_RDFS,
    EXACT_LITERAL_PRESERVATION,
    GEOSPARQL,
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
    CapabilityRole,
    FacadeError,
    QueryRequest,
};

use crate::{
    ContractBinding,
    Probe,
    checks,
};

/// The contract of one catalogue capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityContract(CapabilityTerm);

impl CapabilityContract {
    /// The contract of `capability`.
    #[must_use]
    pub const fn new(capability: CapabilityTerm) -> Self {
        Self(capability)
    }

    /// One contract per catalogue capability, in catalogue order.
    pub fn all() -> impl Iterator<Item = Self> {
        ALL_CAPABILITIES.into_iter().map(Self)
    }

    /// The capability under contract.
    #[must_use]
    pub const fn capability(self) -> CapabilityTerm {
        self.0
    }

    /// Opens a fresh store of `binding` and holds it to this contract: the
    /// positive check when the store declares the capability, else the typed
    /// refusal.
    ///
    /// # Panics
    ///
    /// When the store breaks the contract.
    pub async fn verify<B: ContractBinding>(self, binding: &B) {
        let probe = Probe::open(binding).await;
        if probe.declares(self.0) {
            self.exercise(&probe).await;
        } else {
            self.expect_refusal(&probe).await;
        }
    }

    async fn exercise<B: ContractBinding>(self, probe: &Probe<'_, B>) {
        match self.0 {
            term if term == SPARQL11_QUERY => checks::sparql11_query(probe).await,
            term if term == SPARQL11_UPDATE => checks::sparql11_update(probe).await,
            term if term == SPARQL12_QUERY => checks::sparql12_query(probe).await,
            term if term == SPARQL12_UPDATE => checks::sparql12_update(probe).await,
            term if term == RDF12_TERMS => checks::rdf12_terms(probe).await,
            term if term == EXACT_LITERAL_PRESERVATION => {
                checks::exact_literal_preservation(probe).await;
            }
            term if term == TRANSACTIONS => checks::transactions(probe).await,
            term if term == NAMED_GRAPHS => checks::named_graphs(probe).await,
            term if term == UNION_DEFAULT_GRAPH => checks::union_default_graph(probe).await,
            term if term == GEOSPARQL => checks::geosparql(probe).await,
            term if term == PER_QUERY_TIMEOUT => checks::per_query_timeout(probe).await,
            term if term == ENTAILMENT_RDFS => checks::rdfs(probe).await,
            term if term == ENTAILMENT_OWL_RDF_BASED => checks::owl_rdf_based(probe).await,
            // `GRAPH_STORE_PROTOCOL`, `FULL_TEXT_SEARCH` and any later term.
            term => checks::unverifiable(term),
        }
    }

    async fn expect_refusal<B: ContractBinding>(self, probe: &Probe<'_, B>) {
        for request in self.requests_requiring(probe) {
            let refusal = probe.run(request).await;
            let Err(FacadeError::MissingCapabilities(missing)) = refusal else {
                panic!(
                    "an undeclared <{}> must be refused with missing-capabilities, got {refusal:?}",
                    self.0.capability()
                );
            };
            assert!(
                missing.capabilities().contains(&self.0.capability()),
                "the refusal names <{}>: {missing}",
                self.0.capability()
            );
        }
    }

    /// Requests that require this capability: one in it as a language, or one
    /// requiring it as a feature (and, for a timeout, one carrying a timeout).
    fn requests_requiring<B: ContractBinding>(self, probe: &Probe<'_, B>) -> Vec<QueryRequest> {
        // bounded: at most two requests.
        if self.0.capability().role() == CapabilityRole::Language {
            return vec![probe.request(self.0, "ASK {}")];
        }
        let requiring = probe
            .request(SPARQL11_QUERY, "ASK {}")
            .requiring(self.0)
            .expect("one required feature is within the limit");
        if self.0 == PER_QUERY_TIMEOUT {
            vec![
                requiring,
                probe.timed_request(SPARQL11_QUERY, "ASK {}", 1_000),
            ]
        } else {
            vec![requiring]
        }
    }
}
