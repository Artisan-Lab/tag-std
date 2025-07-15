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
    // safety-lib compiled on host target
    let safety_lib = lib.join("libsafety_lib.rlib");

    make_args(&[
        "-L/home/gh-zjp-CN/tag-std/safety-tool/safety-lib/",
        "--extern=safety=/home/gh-zjp-CN/tag-std/safety-tool/safety-lib/libsafety_lib.rlib",
        // inject safety_lib dependency
        "-L",
        lib.as_str(),
        // Specify direct dependency to allow `use safety_macro` in crate root.
        // The use extern crate syntax only works after --edition=2018.
        &format!("--extern=safety={safety_lib}"),
        // "--extern=safety=/home/gh-zjp-CN/tag-std/safety-tool/safety-lib/libsafety_lib.rlib",
        // inject rapx tool attr
        // "-Zallow-features=register_tool",
        "-Zcrate-attr=feature(register_tool)",
        "-Zcrate-attr=register_tool(rapx)",
    ])
}
