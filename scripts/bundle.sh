#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

cargo build --release -p vm-cli
mkdir -p dist

BIN_NAME="vm-cli"
if [[ "${OS:-}" == "Windows_NT" ]]; then
  BIN_NAME="vm-cli.exe"
fi

cp "target/release/$BIN_NAME" "dist/jevan-kernel-cli${BIN_NAME#vm-cli}"
echo "Bundled: dist/jevan-kernel-cli${BIN_NAME#vm-cli}"
