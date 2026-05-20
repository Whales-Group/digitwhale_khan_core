#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

UEFI_FW="/opt/homebrew/share/qemu/edk2-aarch64-code.fd"
KERNEL="target/aarch64-unknown-uefi/debug/kernel-baremetal.efi"

if [ ! -f "$KERNEL" ]; then
    echo "Kernel not found. Run ./scripts/build-uefi.sh first."
    exit 1
fi

if [ ! -f "$UEFI_FW" ]; then
    echo "UEFI firmware not found at $UEFI_FW"
    exit 1
fi

echo "==> Booting JevKen in QEMU (aarch64)..."
exec qemu-system-aarch64 \
    -M virt \
    -cpu max \
    -m 512M \
    -bios "$UEFI_FW" \
    -kernel "$KERNEL" \
    -nographic
