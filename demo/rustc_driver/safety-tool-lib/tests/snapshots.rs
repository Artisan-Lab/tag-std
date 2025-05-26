use expect_test::expect_file;
use std::{path::Path, process::Command};

fn snapshot(test: &str) {
    let stdout = Command::new("cargo")
        .args(["expand", "--test", test])
        .output()
        .unwrap_or_else(|err| panic!("Failed to run {test}:\n{err}"));
    let expanded = std::str::from_utf8(&stdout.stdout).unwrap();
    let dir = Path::new("snapshots");
    let path = {
        let mut p = dir.join(test);
        assert!(p.set_extension(".rs"));
        p
    };
    expect_file![path].assert_eq(expanded);
}

#[test]
fn memo_arg() {
    snapshot("testcase_memo_arg");
}

#[test]
fn memo_property() {
    snapshot("testcase_memo_property");
}

#[test]
fn property() {
    snapshot("testcase_property");
}
