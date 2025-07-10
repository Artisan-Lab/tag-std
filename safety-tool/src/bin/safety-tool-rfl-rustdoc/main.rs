use eyre::Result;

#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod run_rustdoc;

fn main() -> Result<()> {
    safety_tool::logger::init();

    let args: Vec<_> = std::env::args().skip(1).collect();
    ensure!(!args.is_empty(), "Must pass at least one argument.");

    run_rustdoc::run(args)?;

    Ok(())
}
