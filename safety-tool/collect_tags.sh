#!/bin/bash
set -exuo pipefail

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
