use crate::Utf8PathBuf;

// FIXME: also consider runtime SAFETY_TOOL_SYSROOT

// Usually the paths are subdirectories to `target/safety-tool`.

pub fn root() -> Utf8PathBuf {
    Utf8PathBuf::from(env!("SAFETY_TOOL_SYSROOT"))
}

pub fn lib() -> Utf8PathBuf {
    root().join("lib")
}

pub fn bin() -> Utf8PathBuf {
    root().join("bin")
}

/// binaries under sysroot/bin folder
pub const SAFETY_TOOL: &str = "safety-tool";
pub const SAFETY_TOOL_RFL: &str = "safety-tool-rfl";
pub const CARGO_SAFETY_TOOL: &str = "cargo-safety-tool";

pub fn bin_safety_tool() -> Utf8PathBuf {
    bin().join(SAFETY_TOOL)
}

pub fn bin_safety_tool_rfl() -> Utf8PathBuf {
    bin().join(SAFETY_TOOL_RFL)
}

pub fn bin_cargo_safety_tool() -> Utf8PathBuf {
    bin().join(CARGO_SAFETY_TOOL)
}
