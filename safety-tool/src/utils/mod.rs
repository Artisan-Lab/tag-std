pub mod cmd;
pub mod sysroot;

/// Detect if `VERIFY_RUST_STD` is set. When build for VERIFY_RUST_STD
/// * don't copy libsafety_macro.so to libsafety.so, because verify-rust-std has such crate.
/// * don't set `--extern=safety`.
pub fn verify_rust_std() -> bool {
    std::env::var("VERIFY_RUST_STD").map(|s| s != "0").unwrap_or(false)
}
