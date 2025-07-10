#![feature(let_chains)]
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use eyre::Result;

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod cargo_build;
mod run_safety_tool;

fn main() -> Result<()> {
    safety_tool::logger::init();

    let args: Vec<_> = std::env::args().skip(1).collect();
    ensure!(!args.is_empty(), "Must pass at least one argument.");

    if args[0] == "build-dev" {
        cargo_build::dev()?;
    } else {
        run_safety_tool::run(args)?;
    }

    Ok(())
}
