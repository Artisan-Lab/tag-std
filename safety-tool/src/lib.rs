pub mod logger;
pub mod utils;

pub use camino::{Utf8Path, Utf8PathBuf};
pub use eyre::Result;

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

// NOTE: before compilation (i.e. calling `cargo build` or something)
// `./gen_rust_toolchain_toml.rs $proj` should be run first
// where $proj is one of std, rfl, or asterinas.
crossfig::alias! {
    // verify-rust-std
    pub std: { #[cfg(feature = "std")] },
    // Rust for Linux
    pub rfl: { #[cfg(feature = "rfl")] },
    // Asterinas OS
    pub asterinas: { #[cfg(feature = "asterinas")] }
}
