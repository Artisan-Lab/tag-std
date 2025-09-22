#!/usr/bin/bash

# Set up toolchain
cd safety-tool || exit
rm -f rust-toolchain.toml
./gen_rust_toolchain_toml.rs "${PROJ}"
rustup show

# Build safety-lsp
cd safety-lsp || exit
cargo b
