use crate::Result;
use safety_tool::utils::{
    cmd::{execute, make_args},
    sysroot,
};

// RUSTC_BOOTSTRAP=1 safety-tool tests/snippets/safety_lib_basic.rs -L target/safety-tool/lib/
// -Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_tool(rapx)" --crate-type=lib
pub fn run(mut args: Vec<String>) -> Result<()> {
    if !args.iter().any(|arg| arg.starts_with("--crate-type")) {
        // default to compile lib crate unless `CRATE_TYPE=bin` exists
        let crate_type = if std::env::var("CRATE_TYPE").map(|s| s == "bin").unwrap_or(false) {
            "--crate-type=bin"
        } else {
            "--crate-type=lib"
        };
        args.push(crate_type.to_owned());
    }

    let bin_safety_tool = sysroot::bin_safety_tool();
    if matches!(&*args[0], "--version" | "-V" | "-Vv" | "-vV") {
        // linux will parse the version output
        execute(bin_safety_tool.as_str(), &args, vec![])?;
    } else {
        args.extend(extra_rustc_args());
        info!("args = {args:#?}");
        let vars = vec![("RUSTC_BOOTSTRAP", "1")];
        execute(bin_safety_tool.as_str(), &args, vars)?;
    }

    Ok(())
}

fn extra_rustc_args() -> Vec<String> {
    let safety_lib = sysroot::lib();

    make_args(&[
        // inject safety_lib dependency
        "-L",
        safety_lib.as_str(),
        // inject rapx tool attr
        "-Zcrate-attr=feature(register_tool)",
        "-Zcrate-attr=register_tool(rapx)",
    ])
}
