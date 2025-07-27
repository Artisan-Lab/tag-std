//  RUSTC_BOOTSTRAP=1 rustc a.rs -L target/safety-tool/lib/ -Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_too
// l(rapx)" -o target/a

use camino::Utf8PathBuf;
use eyre::Result;
use safety_tool::{
    logger,
    utils::{
        cmd::{execute, make_args},
        sysroot,
    },
};
use std::sync::LazyLock;

struct Global {
    safety_tool_rfl: String,
}

static GLOBAL: LazyLock<Global> = LazyLock::new(|| {
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

    // Check these paths exist through canonicalize_utf8.
    let make_sure_exists = |path: Utf8PathBuf| path.canonicalize_utf8().unwrap().into_string();

    let _safety_tool = make_sure_exists(sysroot::bin_safety_tool());
    let safety_tool_rfl = make_sure_exists(sysroot::bin_safety_tool_rfl());
    Global { safety_tool_rfl }
});

fn init() {
    _ = &*GLOBAL;
}

#[test]
fn basic() -> Result<()> {
    init();

    let args = make_args(&["tests/snippets/safety_macro_basic.rs"]);

    execute(&GLOBAL.safety_tool_rfl, &args, vec![])?;
    Ok(())
}
