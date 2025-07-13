#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use safety_parser;

/// Safety macros.
pub mod safety;
pub use safety::*;
