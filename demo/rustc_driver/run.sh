set -ex

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo build
export SAFE_TOOL=$PWD/target/debug/safe-tool
export CARGO_SAFE_TOOL=$PWD/target/debug/cargo-safe-tool

# Test basic demo
cd ./tests/basic

cargo clean

# Analyze the lib and bin crates.
# Same as `cargo safe-tool` when tag-std and cargo-safe-tool are installed.
$CARGO_SAFE_TOOL
