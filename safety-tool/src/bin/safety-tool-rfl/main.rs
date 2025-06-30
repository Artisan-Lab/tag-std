use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use eyre::Result;

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod cargo_build;

fn main() -> Result<()> {
    safety_tool::logger::init();
    cargo_build::run()?;

    Ok(())
}
