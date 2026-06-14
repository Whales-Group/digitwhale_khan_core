# Contributing to JevKen

Thanks for your interest. This is a small, focused kernel project. Here's how to contribute effectively.

## Quick start

```bash
# Clone and build
git clone https://github.com/digitwhale/digitwhale_khan_core
cd digitwhale_khan_core
rustup target add aarch64-unknown-uefi
cargo build --target aarch64-unknown-uefi

# Run in QEMU
./scripts/run-qemu.sh

# Run native tests (macOS)
cargo test --target aarch64-apple-darwin
```

## What we need help with

- **Testing** — running the kernel on real AArch64 hardware
- **Documentation** — improving inline docs and architecture notes
- **Drivers** — additional UEFI-compatible hardware support
- **Tooling** — better QEMU scripts, debugger integration

## Guidelines

- Format your code: `cargo fmt`
- No `unsafe` unless absolutely necessary and justified in a comment
- Keep the kernel `no_std` compatible
- Tests for core algorithms are required
- Open an issue before starting significant work

## PR process

1. Open an issue describing what you're fixing or adding
2. Fork and create a branch
3. Submit a PR referencing the issue
4. Ensure CI passes (build + format + tests)
