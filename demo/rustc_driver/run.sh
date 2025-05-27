set -ex
set -o pipefail

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo fmt --check --all
cargo clippy --workspace -- -D clippy::all

cargo build
export SAFE_TOOL=$PWD/target/debug/safe-tool
export CARGO_SAFE_TOOL=$PWD/target/debug/cargo-safe-tool

pushd safety-tool-lib
cargo test
popd

pushd safety-tool-macro
cargo test
popd

# Test basic demo
pushd ./tests/basic

cargo clean

# Analyze the lib and bin crates.
# Same as `cargo safe-tool` when tag-std and cargo-safe-tool are installed.
CARGO_TERM_PROGRESS_WHEN=never $CARGO_SAFE_TOOL | tee macro-expanded/cargo-safe-tool.txt
cargo expand --lib >macro-expanded/lib.rs
