# store-facade

[![CI](https://github.com/legra-ai/store-facade/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/store-facade/actions/workflows/ci.yml)

An open-source Rust facade over query stores. Callers address a logical store
only by its Store IRI and hold a small RDF description of it: its slug, its
labels and its capabilities. The technology behind a store stays hidden.

| Crate | Purpose |
| ----- | ------- |
| [`store-facade-model`](crates/store-facade-model) | Store identity, RDF store descriptions, the capability vocabulary and the streaming query contract |

See [decision 0001](docs/decisions/0001-store-facade-scope-and-naming.md) for
the scope and the naming ruling.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
