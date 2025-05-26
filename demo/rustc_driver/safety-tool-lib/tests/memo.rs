use expect_test::expect_file;
use std::process::Command;

#[test]
fn basic() {
    let stdout =
        Command::new("cargo").args(["expand", "--test", "memo_testcase"]).output().unwrap();
    let expanded = std::str::from_utf8(&stdout.stdout).unwrap();
    expect_file!["snapshots/memo_testcase.rs"].assert_eq(expanded);
}
