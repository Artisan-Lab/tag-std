#![cfg_attr(feature = "asterinas", feature(let_chains))]

pub mod logger;
pub mod stat;
pub mod utils;

pub use camino::{Utf8Path, Utf8PathBuf};
pub use eyre::Result;

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;
