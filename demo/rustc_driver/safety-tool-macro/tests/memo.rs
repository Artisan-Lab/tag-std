use std::process::Command;

#[test]
fn basic() {
    let stdout = Command::new("cargo-expand").args(["--test", "memo_testcase"]).output().unwrap();
    let expanded = std::str::from_utf8(&stdout.stdout).unwrap();
    println!("{expanded}");
}
