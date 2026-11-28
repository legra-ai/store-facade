# store-facade-oxigraph

A [store-facade](https://github.com/legra-ai/store-facade) binding that
answers queries from an embedded [Oxigraph](https://oxigraph.org/) store, in
memory or on disk.

An operator binds a Store IRI to the `oxigraph` implementation with a
[`StoreBinding`](store_facade_model::StoreBinding) carrying an
[`OxigraphConfig`]; [`OxigraphBinder`] opens it. Callers see only the
derived description, which declares:

| Capability | Declared |
| ---------- | -------- |
| `sd:SPARQL11Query`, `sd:SPARQL11Update` | always |
| `sf:SPARQL12Query`, `sf:SPARQL12Update`, `sf:RDF12Terms` | always |
| `sf:NamedGraphs` | always |
| `sf:Transactions` | always: each update applies atomically |
| `sf:PerQueryTimeout` | always |
| `sd:UnionDefaultGraph` | when configured |
| `sf:ExactLiteralPreservation` | never: typed literals return in canonical form |

- **Streaming.** A query evaluates on a blocking thread that feeds a bounded
  channel of [`ResultBuffer`] items and blocks while it is full, so memory
  stays bounded however large the result is.
- **Cancellation and timeouts.** Cancelling the caller's token, the
  request's [`QueryTimeout`](store_facade_model::QueryTimeout) elapsing, or
  dropping the stream stops the evaluation, which is checked between rows
  and inside the evaluator. An update that is interrupted before it commits
  leaves the store unchanged.
- **Errors** map to the model's stable codes: `invalid-query` for syntax
  errors, `cancelled`, `timeout`, and `backend` with a technology-neutral
  message.

The crate passes the `store-facade-contract` suite with in-memory and on-disk
storage, with and without a union default graph.

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.
