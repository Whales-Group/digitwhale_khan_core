#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Building JevKen for aarch64-unknown-uefi..."
cargo build --target aarch64-unknown-uefi "$@"
echo "==> Done. Binary: target/aarch64-unknown-uefi/debug/kernel-baremetal.efi"
echo "    Size: $(wc -c < target/aarch64-unknown-uefi/debug/kernel-baremetal.efi | tr -d ' ') bytes"
