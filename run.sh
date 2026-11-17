#!/usr/bin/env bash
set -euo pipefail
export RUSTUP_TOOLCHAIN=nightly-2026-07-21
cd "$(dirname "$0")/kernel"
exec cargo osdk "$@"
