#!/bin/bash
set -exuo pipefail

./gen_rust_toolchain_toml.rs asterinas
cargo install --path . --locked --debug -Fasterinas

export UPG_DIR=$PWD/target/upg
rm "$UPG_DIR" -rf
mkdir "$UPG_DIR"

# Switch to collect mode.
export SAFETY_TOOL_JSON=1

pushd ./tests/demo/
cargo clean
cargo safety-tool
cp "$UPG_DIR" . -r
popd

pushd ../target-projects/asterinas/tag-asterinas/ostd/
cargo clean
cargo safety-tool --target x86_64-unknown-none
popd

cp "$UPG_DIR"/_tags/ostd.json assets/collect_tags/
