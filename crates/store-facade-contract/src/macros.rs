/// Expands to one `#[tokio::test]` per catalogue capability in a module
/// named `$module`, each constructing its binding with `$binding`.
///
/// ```rust,ignore
/// store_facade_contract::contract_tests!(memory, MemoryBinding::new());
/// ```
#[macro_export]
macro_rules! contract_tests {
    ($module:ident, $binding:expr) => {
        mod $module {
            #[allow(unused_imports)]
            use super::*;

            $crate::contract_tests!(@test sparql11_query, SPARQL11_QUERY, $binding);
            $crate::contract_tests!(@test sparql11_update, SPARQL11_UPDATE, $binding);
            $crate::contract_tests!(@test sparql12_query, SPARQL12_QUERY, $binding);
            $crate::contract_tests!(@test sparql12_update, SPARQL12_UPDATE, $binding);
            $crate::contract_tests!(@test graph_store_protocol, GRAPH_STORE_PROTOCOL, $binding);
            $crate::contract_tests!(@test rdf12_terms, RDF12_TERMS, $binding);
            $crate::contract_tests!(
                @test exact_literal_preservation, EXACT_LITERAL_PRESERVATION, $binding
            );
            $crate::contract_tests!(@test transactions, TRANSACTIONS, $binding);
            $crate::contract_tests!(@test named_graphs, NAMED_GRAPHS, $binding);
            $crate::contract_tests!(@test union_default_graph, UNION_DEFAULT_GRAPH, $binding);
            $crate::contract_tests!(@test full_text_search, FULL_TEXT_SEARCH, $binding);
            $crate::contract_tests!(@test geosparql, GEOSPARQL, $binding);
            $crate::contract_tests!(@test per_query_timeout, PER_QUERY_TIMEOUT, $binding);
            $crate::contract_tests!(@test entailment_rdfs, ENTAILMENT_RDFS, $binding);
            $crate::contract_tests!(
                @test entailment_owl_rdf_based, ENTAILMENT_OWL_RDF_BASED, $binding
            );

            #[test]
            fn covers_the_whole_catalogue() {
                assert_eq!($crate::CapabilityContract::all().count(), 15);
            }
        }
    };
    (@test $name:ident, $term:ident, $binding:expr) => {
        #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
        async fn $name() {
            let binding = $binding;
            $crate::CapabilityContract::new($crate::__vocabulary::$term)
                .verify(&binding)
                .await;
        }
    };
}
