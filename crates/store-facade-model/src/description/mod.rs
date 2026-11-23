//! Store descriptions: the small RDF graph a caller holds about one store.

mod builder;
mod label;
mod labels;
mod limit;
mod reader;
mod store_description;
mod wire;
mod writer;

pub use builder::StoreDescriptionBuilder;
pub use label::{
    LabelText,
    StoreLabel,
};
pub use labels::StoreLabels;
pub use limit::DescriptionLimit;
pub use reader::StoreDescriptionReader;
pub use store_description::StoreDescription;
