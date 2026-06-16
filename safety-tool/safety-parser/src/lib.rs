pub use proc_macro2;
pub use quote;
pub use syn;

/// Parse `#[safety]`.
pub mod safety;

/// Output JSON.
pub mod json;

/// SP configuration, especially definitions.
pub mod configuration;
use configuration::Str;

pub mod split_attrs;
