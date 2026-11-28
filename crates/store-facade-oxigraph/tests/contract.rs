//! The store-facade contract suite, against every storage and graph mode.

mod support;

use support::OxigraphContract;

store_facade_contract::contract_tests!(memory, OxigraphContract::memory());
store_facade_contract::contract_tests!(disk, OxigraphContract::disk());
store_facade_contract::contract_tests!(
    memory_union,
    OxigraphContract::memory().with_union_default_graph()
);
store_facade_contract::contract_tests!(
    disk_union,
    OxigraphContract::disk().with_union_default_graph()
);
