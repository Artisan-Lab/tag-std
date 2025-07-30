#!/bin/bash
set -ex
set -o pipefail

# NOTE:
# 1. set UPDATE_EXPECT=1 to update snapshot tests
# 2. sqlite3 cache may influence snapshots, so remove them when manually cargo test

# Set up toolchain: works under current folder.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib
# Don't emit rlib files.
export STOP_COMPILATION=1

# ./run.sh -Fstd
FEATURES=${1:-"std"}

./gen_rust_toolchain_toml.rs ${FEATURES}

cargo fmt --check --all
cargo clippy -F${FEATURES} --workspace -- -D clippy::all

cargo build -F${FEATURES}
export SAFETY_TOOL=$PWD/target/debug/safety-tool
export SAFETY_TOOL=$PWD/target/debug/safety-tool
export CARGO_SAFETY_TOOL=$PWD/target/debug/cargo-safety-tool
export DATA_SQLITE3=$PWD/target/data.sqlite3

cargo test -F${FEATURES}

pushd safety-lib
cargo test
popd

pushd safety-macro
cargo test
popd

# enable tag definitions
rm $DATA_SQLITE3
export SP_FILE=$PWD/assets/sp-core.toml

# Test basic demo
pushd ./tests/basic

cargo clean

# Emit artifacts for build scripts.
unset STOP_COMPILATION

# Analyze the lib and bin crates.
PREFIX=$PWD/
CARGO_TERM_PROGRESS_WHEN=never $CARGO_SAFETY_TOOL | sed "s#$PREFIX##g" | tee macro-expanded/cargo-safety-tool.txt
cargo expand --lib >macro-expanded/lib.rs
cargo expand --bin demo >macro-expanded/main.rs
cargo doc --no-deps
