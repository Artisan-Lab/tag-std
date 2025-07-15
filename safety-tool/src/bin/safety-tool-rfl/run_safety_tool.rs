use crate::Result;
use safety_tool::utils::{
    cmd::{execute, make_args},
    sysroot,
};

// RUSTC_BOOTSTRAP=1 safety-tool tests/snippets/safety_lib_basic.rs -L target/safety-tool/lib/
// -Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_tool(rapx)" --crate-type=lib
pub fn run(mut args: Vec<String>) -> Result<()> {
    let bin_safety_tool = sysroot::bin_safety_tool();
    if matches!(&*args[0], "--version" | "-V" | "-Vv" | "-vV") {
        // linux will parse the version output
        execute(bin_safety_tool.as_str(), &args, vec![])?;
    } else {
        if !args.iter().any(|arg| arg.starts_with("--crate-type")) {
            // default to compile lib crate unless `CRATE_TYPE=bin` exists
            let crate_type = if std::env::var("CRATE_TYPE").map(|s| s == "bin").unwrap_or(false) {
                "--crate-type=bin"
            } else {
                "--crate-type=lib"
            };
            args.push(crate_type.to_owned());
        }

        args.extend(extra_rustc_args());
        info!("args = {args:#?}");
        let vars = vec![("RUSTC_BOOTSTRAP", "1")];
        execute(bin_safety_tool.as_str(), &args, vars)?;
    }

    Ok(())
}

fn extra_rustc_args() -> Vec<String> {
    // let lib = sysroot::lib();
    // let safety_lib = lib.join("libsafety_lib.rlib");

    // safety-lib compiled on no-std target
    make_args(&[
        "-L/home/gh-zjp-CN/tag-std/safety-tool/safety-lib/",
        // inject safety_lib dependency
        // "-L",
        // lib.as_str(),
        // NOTE: the last -Zallow-features wins, meaning that specified by rfl
        // previously will be disregarded.
        // cc https://github.com/rust-lang/rust/issues/143312
        // "-Zallow-features=register_tool",
        //
        // Specify direct dependency to allow `use safety_macro` in crate root.
        // The use extern crate syntax only works after --edition=2018.
        // &format!("--extern=safety={safety_lib}"),
        "--extern=safety=/home/gh-zjp-CN/tag-std/safety-tool/safety-lib/libsafety_lib.rlib",
        // inject rapx tool attr
        "-Zcrate-attr=feature(register_tool)",
        "-Zcrate-attr=register_tool(rapx)",
    ])
}

// fn find_safety_lib_rmeta(path: &Utf8Path) -> Result<String> {
//     for entry in std::fs::read_dir(path).unwrap() {
//         let entry = entry?;
//         let file_name = entry.file_name().into_string().unwrap();
//         if file_name.starts_with("libsafety_lib") && file_name.ends_with(".rmeta") {
//             info!(file_name);
//             return Ok(entry.path().canonicalize()?.to_str().unwrap().to_owned());
//         }
//     }
//     Err(eyre!("Can't find libsafety_lib rmeta in {path}"))
// }
