#![feature(let_chains)]
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use eyre::Result;

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod cargo_build;
use cargo_build::CopyMode;

fn main() -> Result<()> {
    safety_tool::logger::init();

    // cp safety-parser's lib
    cargo_build::run("safety-lib", CopyMode::Lib, "safety-lib")?;
    // cp safety-tool's bins
    cargo_build::run(".", CopyMode::Bin, "safety-tool")?;

    Ok(())
}
