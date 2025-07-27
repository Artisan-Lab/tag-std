use crate::Result;
use safety_tool::utils::{
    cmd::{execute, make_args},
    sysroot,
};

// RUSTC_BOOTSTRAP=1 safety-tool tests/snippets/safety_lib_basic.rs -L target/safety-tool/lib/
// -Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_tool(rapx)" --crate-type=lib
pub fn run(mut args: Vec<String>) -> Result<()> {
    let bin_rustdoc = "rustdoc";
    if matches!(&*args[0], "--version" | "-V" | "-Vv" | "-vV") {
        // linux will parse the version output
        execute(bin_rustdoc, &args, vec![])?;
    } else {
        args.extend(extra_rustc_args());
        info!("args = {args:#?}");
        let vars = vec![("RUSTC_BOOTSTRAP", "1")];
        execute(bin_rustdoc, &args, vars)?;
    }

    Ok(())
}

fn extra_rustc_args() -> Vec<String> {
    let lib = sysroot::lib();
    // safety-macro compiled on host target
    let safety_macro = lib.join("libsafety_macro.so");

    make_args(&[
        // inject safety_macro and safety_lib dependency
        "-L",
        lib.as_str(),
        // Specify direct dependency to allow `use safety_macro` in crate root.
        // The use extern crate syntax only works after --edition=2018.
        "--extern=safety_macro",
        // safety is compiled in linux/rust (on no_std target)
        "--extern=safety=./rust/libsafety.rmeta",
        // safety is compiled on host target
        &format!("--extern=safety={safety_macro}"),
        // inject rapx tool attr
        // "-Zallow-features=register_tool",
        "-Zcrate-attr=feature(register_tool)",
        "-Zcrate-attr=register_tool(rapx)",
    ])
}
