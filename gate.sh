#!/usr/bin/env bash
set -euo pipefail
# Host gate (stable 1.100): ostd/kernel 無法在 host target 編譯，故用 -p 白名單
cargo fmt --check
cargo clippy -p arctypon-core -p arctypon-types --all-targets -- -D warnings
cargo test  -p arctypon-core -p arctypon-types
# Kernel gate (nightly 釘版): 交叉目標，由 OSDK 負責
cd kernel
RUSTUP_TOOLCHAIN=nightly-2026-07-21 cargo osdk test
