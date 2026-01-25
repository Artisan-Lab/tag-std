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

# Set up LLVM for Rust for Linux
# We use system LLVM packages instead of downloading prebuilt bundles
llvm=llvm-21.1.4-rust-1.91.0-$(uname -m)
if [ ! -d "${llvm}" ]; then
  # Install LLVM from system packages
  sudo apt-get update
  sudo apt-get install -y llvm-15 llvm-15-dev libclang-15-dev clang-15
  
  # Create a directory structure that mimics the expected layout
  mkdir -p "${llvm}/bin" "${llvm}/lib"
  
  # Create symlinks to system LLVM tools
  # Find all llvm-*-15 executables and create symlinks without the version suffix
  find /usr/bin -name 'llvm-*-15' -type f -executable 2>/dev/null | while read -r tool; do
    basename=$(basename "$tool" | sed 's/-15$//')
    ln -sf "$tool" "${llvm}/bin/${basename}"
  done
  
  # Link clang without version suffix
  ln -sf /usr/bin/clang-15 "${llvm}/bin/clang"
  ln -sf /usr/bin/clang++-15 "${llvm}/bin/clang++"
  
  # Link libclang - try multiple common locations
  libclang_found=false
  for libclang_path in \
    /usr/lib/llvm-15/lib/libclang.so \
    /usr/lib/x86_64-linux-gnu/libclang-15.so.1 \
    /usr/lib/aarch64-linux-gnu/libclang-15.so.1 \
    /usr/lib/libclang-15.so.1 \
    /usr/lib/libclang.so.1; do
    if [ -f "$libclang_path" ]; then
      ln -sf "$libclang_path" "${llvm}/lib/libclang.so"
      libclang_found=true
      echo "Found libclang at: $libclang_path"
      break
    fi
  done
  
  if [ "$libclang_found" = false ]; then
    echo "Warning: libclang not found, build may fail"
  fi
  
  echo "LLVM 15 installed from system packages"
else
  echo "llvm and rust toolchain have been downloaded"
fi
