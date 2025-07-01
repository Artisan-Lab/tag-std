//  RUSTC_BOOTSTRAP=1 rustc a.rs -L target/safety-tool/lib/ -Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_too
// l(rapx)" -o target/a

use eyre::Result;
use safety_tool::{
    logger,
    utils::cmd::{execute, make_args},
};
use std::sync::LazyLock;

static INIT: LazyLock<()> = LazyLock::new(|| {
    logger::init();
    let mut cmd = assert_cmd::Command::cargo_bin("safety-tool-rfl").unwrap();
    let output = cmd.arg("build-dev").output().unwrap();
    assert!(
        output.status.success(),
        "Failed to run `safety-tool-rfl build-dev`:\nstdout={stdout:?}\nstderr={stderr}",
        stdout = String::from_utf8_lossy(&output.stdout),
        stderr = String::from_utf8_lossy(&output.stderr)
    );
    println!("Success to build safety-tool artifacts.");
});

fn init() {
    _ = &*INIT;
}

#[test]
fn basic() -> Result<()> {
    init();

    let args = make_args(&["tests/snippets/safety_lib_basic.rs"]);
    execute("safety-tool-rfl", &args, vec![])?;
    Ok(())
}
