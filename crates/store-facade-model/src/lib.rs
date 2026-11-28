#![doc = include_str!("../README.md")]

mod binding;
mod cache;
mod capability;
mod description;
mod error;
mod identity;
mod query;
pub mod vocabulary;

pub use binding::{
    Binder,
    ImplementationName,
    StoreBinding,
};
pub use cache::{
    CacheKey,
    EntityTag,
    Validator,
};
pub use capability::{
    Capability,
    CapabilityRole,
    CapabilitySet,
    MissingCapabilities,
    RequiredCapabilities,
};
pub use description::{
    DescriptionLimit,
    LabelText,
    StoreDescription,
    StoreDescriptionBuilder,
    StoreDescriptionReader,
    StoreLabel,
    StoreLabels,
};
pub use error::{
    ErrorCode,
    FacadeError,
    SlugRejection,
};
pub use identity::{
    StoreIri,
    StoreSlug,
};
pub use query::{
    CancellableStream,
    QueryLanguage,
    QueryRequest,
    QueryStore,
    QueryText,
    QueryTimeout,
};
