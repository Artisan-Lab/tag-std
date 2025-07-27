use expect_test::expect_file;
use std::{path::Path, process::Command};

fn snapshot(test: &str) {
    let err = |e: &dyn std::fmt::Display| panic!("Failed to run `cargo expand --test {test}:\n{e}");
    let stdout =
        Command::new("cargo").args(["expand", "--test", test]).output().unwrap_or_else(|e| err(&e));
    if !stdout.status.success() {
        let e = std::str::from_utf8(&stdout.stderr).unwrap();
        err(&e);
    }

    let expanded = std::str::from_utf8(&stdout.stdout).unwrap();
    let dir = Path::new("snapshots");
    let path = {
        let mut p = dir.join(test);
        assert!(p.set_extension("rs"));
        p
    };
    expect_file![path].assert_eq(expanded);
}

#[test]
fn safety_macro() {
    snapshot("testcase_safety_macro");
}
