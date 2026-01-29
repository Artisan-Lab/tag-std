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
  git -C linux checkout FETCH_HEAD # c3963b1844a3
else
  echo "linux source code has been downloaded"
fi

# Note: No longer downloading custom LLVM toolchain.
# The build now uses the system's LLVM via LLVM=1 make flag.
# bindgen-cli will be installed in check.sh using cargo.
