# Release approval

Every minor and major version increase requires the user's explicit approval of
the exact level, version and release scope before changing that number or
publishing it. General permission to implement, merge, maintain, or release does
not suffice. Preserve accurate breaking-change markers and stop at the
release-approval gate. Never approve an environment through owner credentials
without that specific user authorization. Patch releases are fully automatic.

## Zero-user pre-production versions

This repository has zero users. Fixes use patch releases; features and breaking
changes use minor releases. Preserve accurate breaking-change markers and use
`breaking-change-bump: minor` for both planning and preparation. Those minors,
like every minor or major increase, still require the user's specific explicit
approval. Do not add migrations or
compatibility paths for hypothetical consumers.

# Commit policy

Conventional commits with exactly one scope from this list, enforced in shape
by `.githooks/commit-msg` and on PR titles by CI:

| Scope | Covers |
| ----- | ------ |
| `model` | the `store-facade-model` crate |
| `vocabulary` | capability constants and the vendored Turtle vocabulary |
| `repo` | repository layout, licenses, README, agent instructions |
| `docs` | decision records and other documentation |
| `ci` | workflows and policy scripts |
| `hooks` | Git hooks and the formatter wrapper |
| `deps` | dependency updates (Dependabot uses `build(deps)` and `ci(deps)`) |
| `release` | release preparation commits from automation |

Later crates add their own scope: `service` for `store-facade`, and the
backend name for each `store-facade-<backend>`. Never bypass hooks, and
every commit is signed.

# Naming

Use "store-facade" everywhere ([decision 0001](docs/decisions/0001-store-facade-scope-and-naming.md)):
`store-facade-model`, the service `store-facade`, backends
`store-facade-<backend>`, and the same names for Python wheels.

# Code conventions

- Edition 2024; `missing_docs`, `clippy::all` and `clippy::pedantic` deny.
- Strict newtypes for every constrained value (`StoreIri`, `StoreSlug`,
  `Capability`, `QueryText`); construction and validation live on the type
  (`try_new`, builders), behaviour on `impl` methods.
- Source files stay under 400 lines; a `mod.rs` holds only its docstring,
  `mod` declarations and re-exports.
- No `serde_json::json!` and no `serde_json::Value` in public APIs.
- Bounded memory: every collection that grows from input carries a limit and a
  `// bounded:` note; RDF is read and written one triple at a time.
- A description never names the technology, cluster, topology, region or
  lifecycle behind a store. Design from public W3C specifications only.
