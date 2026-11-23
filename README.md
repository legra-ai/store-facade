# store-facade

An open-source Rust facade over query stores. Callers address a logical store
only by its Store IRI and hold a small RDF description of it: its slug, its
labels and its capabilities. The technology behind a store stays hidden.

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
