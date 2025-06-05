set -ex
set -o pipefail

# NOTE:
# 1. set UPDATE_EXPECT=1 to update snapshot tests
# 2. sqlite3 cache may influence snapshots, so remove them when manually cargo test

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib
# Don't emit rlib files.
export STOP_COMPILATION=1

cargo fmt --check --all
cargo clippy --workspace -- -D clippy::all

cargo build
export SAFETY_TOOL=$PWD/target/debug/safety-tool
export CARGO_SAFETY_TOOL=$PWD/target/debug/cargo-safety-tool

rm data.sqlite3 a.sqlite3 -f
cargo test
rm data.sqlite3 a.sqlite3 -f

pushd safety-tool-lib
cargo test
popd

pushd safety-tool-macro
cargo test
popd

# Test basic demo
pushd ./tests/basic

rm data.sqlite3 -f
cargo clean

# Emit artifacts for build scripts.
unset STOP_COMPILATION

# Analyze the lib and bin crates.
# Same as `cargo safe-tool` when tag-std and cargo-safe-tool are installed.
CARGO_TERM_PROGRESS_WHEN=never $CARGO_SAFETY_TOOL | tee macro-expanded/cargo-safe-tool.txt
cargo expand --lib >macro-expanded/lib.rs
cargo expand --bin demo >macro-expanded/main.rs
rm data.sqlite3 -f
