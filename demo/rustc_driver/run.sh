set -ex

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo build
export TAG_STD=$PWD/target/debug/tag-std
export CARGO_TAG_STD=$PWD/target/debug/cargo-tag-std
# export CARGO_TERM_VERBOSE=true

# Test basic demo
cd ./tests/basic

cargo clean

# Analyze the lib crate.
# $tag_std src/lib.rs --crate-type=lib

# Analyze the bin crate.
$CARGO_TAG_STD
