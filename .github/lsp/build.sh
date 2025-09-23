#!/usr/bin/bash

set -exou pipefail

# Set up toolchain
cd safety-tool
rm -f rust-toolchain.toml
./gen_rust_toolchain_toml.rs "${PROJ}"
rustup show

# Build safety-lsp
cd safety-lsp
cargo b
