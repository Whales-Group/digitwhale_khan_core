#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Running native tests..."
cargo test --target aarch64-apple-darwin "$@"
