//! Operator-side bindings: which implementation, configured how, serves a
//! Store IRI.
//!
//! A binding is never part of a [`crate::StoreDescription`]; callers see only
//! the description a binder derives from it.

mod binder;
mod implementation_name;
mod store_binding;

pub use binder::Binder;
pub use implementation_name::ImplementationName;
pub use store_binding::StoreBinding;
