#!/bin/bash

# modified from https://github.com/rust-lang/rust/blob/bc4376fa73b636eb6f2c7d48b1f731d70f022c4b/src/ci/docker/scripts/rfl-build.sh

set -exou pipefail

# Linux version
LINUX_REPO=https://github.com/Artisan-Lab/tag-rust-for-linux.git

# Download Linux at a specific commit
mkdir -p linux
git -C linux init
git -C linux remote add origin ${LINUX_REPO}
git -C linux fetch --depth 1 origin rust-next
git -C linux checkout FETCH_HEAD

# Download LLVM and rustc toolchain required by Rust for Linux
# see https://mirrors.edge.kernel.org/pub/tools/llvm/rust/
llvm=llvm-20.1.7-rust-1.87.0-$(uname -m)
wget https://mirrors.edge.kernel.org/pub/tools/llvm/rust/files/$llvm.tar.xz
tar -xvf $llvm.tar.xz
