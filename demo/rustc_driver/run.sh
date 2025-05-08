set -ex

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo build
export TAG_STD=$PWD/target/debug/tag-std
export CARGO_TAG_STD=$PWD/target/debug/cargo_tag_std
export TAG_STD_CROSS_CRATES=true
export CARGO_TERM_VERBOSE=true

# Test basic demo
cd ./tests/basic

cargo b

# Analyze the lib crate.
# $tag_std src/lib.rs --crate-type=lib

# Analyze the bin crate.
$CARGO_TAG_STD
