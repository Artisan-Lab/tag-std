#!/bin/bash
set -ex
set -o pipefail

# NOTE:
# 1. set UPDATE_EXPECT=1 to update snapshot tests
# 2. sqlite3 cache may influence snapshots, so remove them when manually cargo test

# Fetch latest stat JSON for asternias and rust-for-linux
gh run download -D assets/stat/ -n "stat_asterinas" -n "stat_rfl-X64"

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib
# Don't emit rlib files.
export STOP_COMPILATION=1

# ./run.sh -Fstd
FEATURES=${1:-"std"}

# Switch toolchain
./gen_rust_toolchain_toml.rs "${FEATURES}"

# Remove data.sqlite3 the cache
rm -f target/data.sqlite3 tests/demo/data.sqlite3

cargo fmt --check --all
cargo clippy -F "${FEATURES}" --workspace -- -D clippy::all

cargo build -F "${FEATURES}"
export SAFETY_TOOL=$PWD/target/debug/safety-tool
export SAFETY_TOOL=$PWD/target/debug/safety-tool
export CARGO_SAFETY_TOOL=$PWD/target/debug/cargo-safety-tool
export DATA_SQLITE3=$PWD/target/data.sqlite3

cargo test -F "${FEATURES}"

pushd safety-lib
cargo test
popd

pushd safety-macro
cargo test
popd

# enable tag definitions
rm "$DATA_SQLITE3"
export SP_DIR=$PWD/assets

pushd safety-lsp
cargo test
popd

# Test basic demo
pushd ./tests/demo

cargo clean

# Emit artifacts for build scripts.
unset STOP_COMPILATION

# Analyze the lib and bin crates.
export SP_OUT_DIR=$PWD/out
EXPAND_DIR=$SP_OUT_DIR/macro-expanded
rm -rf "$SP_OUT_DIR" # clear outputs
mkdir -p "$EXPAND_DIR"
$CARGO_SAFETY_TOOL # run safety-tool to check
cargo expand --lib >"$EXPAND_DIR"/lib.rs
cargo expand --bin demo >"$EXPAND_DIR"/main.rs
cargo doc --no-deps
