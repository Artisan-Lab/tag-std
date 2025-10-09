#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use safety_parser;

pub use safety_macro::{checked, requires};
