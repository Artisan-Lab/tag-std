use std::{env::var, process::Command};

fn main() {
    // Search cargo-safe-tool and safe-tool CLI through environment variables,
    // or just use the name if absent.
    let cargo_safe_tool =
        &*var("CARGO_SAFETY_TOOL").unwrap_or_else(|_| "cargo-safety-tool".to_owned());
    let safe_tool = &*var("SAFETY_TOOL").unwrap_or_else(|_| "safety-tool".to_owned());

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
        // Entry for cargo-safety-tool: all arguments after `cargo safety-tool`
        // will be passed to `cargo build`.
        let mut args = args;
        if args[0].ends_with("cargo-safety-tool") && args[1] == "safety-tool" {
            // [cargo, safety-tool, args...]
            args.remove(0);
            args[0] = "build".to_owned();
        } else {
            unimplemented!("Need to support this case: {args:#?}")
        }
        // cargo build args...
        run("cargo", &args, &[("RUSTC", cargo_safe_tool), ("WRAPPER", "1")]);
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
