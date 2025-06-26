#!/bin/bash

# modified from https://github.com/rust-lang/rust/blob/bc4376fa73b636eb6f2c7d48b1f731d70f022c4b/src/ci/docker/scripts/rfl-build.sh

set -exou pipefail

# Linux version
LINUX_VERSION=v6.16-rc1

# Download Linux at a specific commit
mkdir -p linux
git -C linux init
git -C linux remote add origin https://github.com/Rust-for-Linux/linux.git
git -C linux fetch --depth 1 origin ${LINUX_VERSION}
git -C linux checkout FETCH_HEAD
