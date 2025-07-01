use crate::Result;
use safety_tool::utils::{
    cmd::{execute, make_args},
    sysroot,
};

// RUSTC_BOOTSTRAP=1 safety-tool tests/snippets/safety_lib_basic.rs -L target/safety-tool/lib/
// -Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_tool(rapx)" --crate-type=lib
pub fn run(mut args: Vec<String>) -> Result<()> {
    args.extend(extra_rustc_args());
    let vars = vec![("RUSTC_BOOTSTRAP", "1")];

    execute("safety-tool", &args, vars)?;

    Ok(())
}

fn extra_rustc_args() -> Vec<String> {
    let safety_lib = sysroot::lib();

    let crate_type = if std::env::var("CRATE_TYPE").map(|s| s == "bin").unwrap_or(false) {
        "--crate-type=bin"
    } else {
        "--crate-type=lib"
    };

    make_args(&[
        // default to compile lib crate unless `CRATE_TYPE=bin` exists
        crate_type,
        // inject safety_lib dependency
        "-L",
        safety_lib.as_str(),
        // inject rapx tool attr
        "-Zcrate-attr=feature(register_tool)",
        "-Zcrate-attr=register_tool(rapx)",
    ])
}
