#!/usr/bin/env bash
set -euo pipefail
cd kernel
cargo osdk run "$@"
