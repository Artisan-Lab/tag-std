use std::{env::var, process::Command};

fn main() {
    // Search cargo-safe-tool and safe-tool CLI through environment variables,
    // or just use the name if absent.
    let cargo_safe_tool = &*var("CARGO_SAFE_TOOL").unwrap_or_else(|_| "cargo-safe-tool".to_owned());
    let safe_tool = &*var("SAFE_TOOL").unwrap_or_else(|_| "safe-tool".to_owned());

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() == 2 && args[1].as_str() == "-vV" {
        // cargo invokes `rustc -vV` first
        run("rustc", &["-vV".to_owned()], &[]);
    } else if std::env::var("WRAPPER").as_deref() == Ok("1") {
        // then cargo invokes `rustc - --crate-name ___ --print=file-names`
        // if args[1] == "-" {
        //     // `rustc -` is a substitute file name from stdin
        //     args[1] = "src/main.rs".to_owned();
        // }

        run(safe_tool, &args[1..], &[]);
    } else {
        run("cargo", &["build"].map(String::from), &[("RUSTC", cargo_safe_tool), ("WRAPPER", "1")]);
    }
}

fn run(cmd: &str, args: &[String], vars: &[(&str, &str)]) {
    let status = Command::new(cmd)
        .args(args)
        .envs(vars.iter().copied())
        .stdout(std::io::stdout())
        .stderr(std::io::stderr())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if !status.success() {
        panic!("[error] {cmd}: args={args:#?} vars={vars:?}");
    }
}
