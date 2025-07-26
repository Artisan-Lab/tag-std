#![feature(let_chains)]

pub use proc_macro2;
pub use quote;
pub use syn;

pub mod property_attr;

/// Parse `#[safety]`.
pub mod safety;

/// SP configuration, especially definitions.
pub mod configuration;
use configuration::Str;
