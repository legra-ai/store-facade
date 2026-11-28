# store-facade-contract

The capability contract suite of
[store-facade](https://github.com/legra-ai/store-facade). It is a development
crate: a binding runs it from its own tests, against its own stores.

The suite has one test per capability in the model's catalogue
(`store_facade_model::vocabulary::ALL_CAPABILITIES`):

- when the store **declares** the capability, the test exercises it and must
  pass: declaring a capability is a promise the contract holds the binding to;
- when it does **not**, the test sends a request requiring the capability and
  expects the typed `missing-capabilities` refusal naming it, before anything
  executes.

A binding implements [`ContractBinding`] for a small test type that opens a
fresh, empty store, seeds it from N-Quads and renders each result item as an
[`Observation`]. [`contract_tests!`] then expands to the whole suite:

```rust,ignore
store_facade_contract::contract_tests!(memory, MemoryBinding::new());
```

Capabilities the model offers no request surface for yet
(`sf:GraphStoreProtocol`, `sf:FullTextSearch`) have no positive check: a
binding that declares one fails its contract test until the model can
express the capability.

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.
