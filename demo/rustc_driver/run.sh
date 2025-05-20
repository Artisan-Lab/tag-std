set -ex

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo build
export TAG_STD=$PWD/target/debug/safe-tool
export CARGO_TAG_STD=$PWD/target/debug/cargo-safe-tool

# Test basic demo
cd ./tests/basic

cargo clean

# Analyze the lib and bin crates.
# Same as `cargo tag-std` when tag-std and cargo-tag-std are installed.
$CARGO_TAG_STD
