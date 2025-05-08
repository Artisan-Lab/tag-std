use std::{env::var, process::Command};

fn main() {
    let cargo_tag_std = var("CARGO_TAG_STD").unwrap();
    let tag_std = var("TAG_STD").unwrap();

    let mut args = std::env::args().collect::<Vec<_>>();

    if args.len() == 2 && args[1].as_str() == "-vV" {
        // cargo invokes `rustc -vV` first
        run("rustc", &["-vV".to_owned()], &[]);
    } else if std::env::var("WRAPPER").as_deref() == Ok("1") {
        // then cargo invokes `rustc - --crate-name ___ --print=file-names`
        if args[1] == "-" {
            // `rustc -` is a substitute file name from stdin
            args[1] = "src/lib.rs".to_owned();
        }

        run(&tag_std, &args[1..], &[]);
    } else {
        run("cargo", &["build"].map(String::from), &[("RUSTC", &cargo_tag_std), ("WRAPPER", "1")]);
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
        eprintln!("[error] {cmd}: args={args:?} vars={vars:?}");
    }
}
