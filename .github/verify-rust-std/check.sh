#!/bin/bash

set -exou pipefail

# Enable verify-rust-std only logics.
export VERIFY_RUST_STD=1

# Install safety-tool
pushd safety-tool
rm -f rust-toolchain.toml
./gen_rust_toolchain_toml.rs std

# Set up dynamic link path for rustc driver.
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo install --path . --locked -Fstd
safety-tool --version
# Generate bin and lib in target/safety-tool
safety-tool-rfl build-dev
popd

# Check libcore
pushd rapx-verify-rust-std/library/core
cargo clean
SAFETY_TOOL=safety-tool-rfl RUSTFLAGS=--cfg=rapx cargo safety-tool
rm ../data.sqlite3
