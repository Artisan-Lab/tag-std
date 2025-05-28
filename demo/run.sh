set -ex
set -o pipefail

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo fmt --check --all
cargo clippy --workspace -- -D clippy::all

cargo build
export SAFE_TOOL=$PWD/target/debug/safe-tool

pushd safety-tool-lib
cargo test
popd

pushd safety-tool-macro
cargo test
popd

# Test basic demo
pushd ./tests
cargo clean
cargo expand --lib > result/lib_after_macro_expansion.rs
