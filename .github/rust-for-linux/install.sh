#!/bin/bash

# modified from https://github.com/rust-lang/rust/blob/bc4376fa73b636eb6f2c7d48b1f731d70f022c4b/src/ci/docker/scripts/rfl-build.sh

set -exou pipefail

# Linux version
LINUX_REPO=https://github.com/Artisan-Lab/tag-rust-for-linux.git
LINUX_BRANCH=rust-next
# === temporary for CI ===
# LINUX_REPO=https://github.com/os-checker/linux
# LINUX_BRANCH=tag-rust-for-linux

# Download Linux at a specific commit
if [ ! -d "linux" ]; then
  mkdir -p linux
  git -C linux init
  git -C linux remote add origin ${LINUX_REPO}
  git -C linux fetch --depth 1 origin ${LINUX_BRANCH}
  git -C linux checkout FETCH_HEAD # b66755f
else
  echo "linux source code has been downloaded"
fi

# Download LLVM and rustc toolchain required by Rust for Linux
# see https://mirrors.edge.kernel.org/pub/tools/llvm/rust/
llvm=llvm-21.1.4-rust-1.91.0-$(uname -m)
if [ ! -d "${llvm}" ]; then
  wget https://mirrors.edge.kernel.org/pub/tools/llvm/rust/files/"$llvm".tar.xz
  tar -xvf "$llvm".tar.xz
else
  echo "llvm and rust toolchain have been downloaded"
fi
