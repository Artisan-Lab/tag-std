set -ex

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo build
tag_std=$PWD/target/debug/tag-std

# Test basic demo
cd ./tests/basic
$tag_std src/lib.rs --crate-type=lib
