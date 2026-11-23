//! Cache hooks: response keys and validators. S1 defines them; no cache uses
//! them yet.

mod entity_tag;
mod key;
mod validator;

pub use entity_tag::EntityTag;
pub use key::CacheKey;
pub use validator::Validator;
