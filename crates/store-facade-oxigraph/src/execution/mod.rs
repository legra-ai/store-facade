//! Running one query or update off the async runtime, with cancellation and
//! timeouts.

mod interruption;
mod producer;
mod update;

pub(crate) use interruption::Interruption;
pub(crate) use producer::{
    ItemSink,
    Producer,
};
pub(crate) use update::UpdateRun;
