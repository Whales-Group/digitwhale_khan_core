# JevKen — KhanOS Kernel

JevKen is the kernel at the heart of KhanOS — a from-scratch baremetal kernel for **AArch64 (ARM64)** written entirely in **Rust** (`no_std`, `no_main`).

It boots via UEFI, manages memory, handles interrupts and exceptions, drives hardware, and runs a round-robin scheduler with ELF-loaded user processes — all on real AArch64 hardware paths, testable in QEMU today.

---

## What's built

| Subsystem | Status |
|---|---|
| **Boot** — UEFI entry, stack setup, framebuffer (GOP) capture, firmware memory map | ✅ Done |
| **Memory** — Bitmap frame allocator, 4-level page tables (identity + higher-half), bump heap allocator, page fault handler | ✅ Done |
| **Interrupts** — AArch64 exception vector table (VBAR_EL1), GICv2 distributor + CPU interface, sync/irq/fiq/serror handlers | ✅ Done |
| **Drivers** — PL011 UART serial (ring buffer + formatted output), GOP framebuffer (put_pixel, put_char, write_str), PL050 KMI keyboard (scancode decode, read_line), PCI ECAM enumeration | ✅ Done |
| **Tasking** — Task state model (Ready/Running/Blocked/Exited), round-robin scheduler (64 tasks, timeslice preemption), context switch (x0–x30, SP, ELR, SPSR), EL0/EL1 privilege separation, SVC syscall dispatch | ✅ Done |
| **ELF Loader** — ELF binary parser, per-task page tables, init process (pid 1) launched by scheduler | ✅ Done |
| **Layered Orchestration** — Deterministic boot order with registered layers, priorities, lifecycle hooks, and locking policy | ✅ Done |

## Architecture

```
firmware (UEFI)
  ↓
_entry (stack + CPU init)
  ↓
kernel_init()
  ├── memory::init_memory()           ← frame allocator, page tables, heap
  ├── interrupts::init_vectors()      ← VBAR_EL1, GICv2
  ├── drivers::init_early()           ← serial, framebuffer, keyboard, PCI
  ├── task::init_scheduler()          ← idle task, scheduler structs
  └── task::spawn(init_main)          ← pid 1 from ELF
       ↓
  scheduler::schedule()               ← round-robin, context switch
```

## Quick start

```bash
# Prerequisites
rustup target add aarch64-unknown-uefi

# Build the kernel
cargo build --target aarch64-unknown-uefi

# Run in QEMU
./scripts/run-qemu.sh

# Run native tests (macOS)
cargo test --target aarch64-apple-darwin
```

The build produces `target/aarch64-unknown-uefi/debug/kernel-baremetal.efi`, a UEFI-compatible binary that boots on QEMU virt (aarch64) with GICv2, PL011 serial, and framebuffer display.

## Project structure

```
digitwhale_khan_core/
├── kernel-baremetal/          # The kernel crate (no_std, no_main)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # UEFI entry point, firmware memory map
│       ├── lib.rs             # Kernel init, module re-exports
│       ├── boot/              # Stack setup, entry point
│       ├── memory/            # Frame allocator, page tables, heap, memmap
│       ├── interrupts/        # Exception vectors, GIC, exception handlers
│       ├── drivers/           # Serial, framebuffer, keyboard, PCI
│       ├── task/              # Scheduler, context switch, state, syscall
│       ├── layered_orchestration/  # Boot layer registration, lifecycle, runner
│       ├── elf.rs             # ELF binary parser and loader
│       ├── init.rs            # Kernel init process (pid 1)
│       └── kernel_log.rs      # Structured serial logging
├── docs/
│   ├── README.md
│   └── kernel/
│       ├── LAYERED_ORCHESTRATION.md
│       ├── BOOT_FLOW.md
│       └── PROGRESS_TRACKER.md
├── scripts/
│   ├── build-uefi.sh
│   ├── run-qemu.sh
│   └── test-native.sh
├── .github/
│   ├── workflows/ci.yml
│   ├── CODEOWNERS
│   ├── CONTRIBUTING.md
│   ├── pull_request_template.md
│   └── ISSUE_TEMPLATE/
└── Cargo.toml                 # Workspace root
```

## Roadmap

| Phase | Focus | Status |
|---|---|---|
| **1 — Foundation** | Boot, memory, interrupts, drivers, scheduling, ELF | ✅ Running in QEMU |
| **2 — Expansion** | Syscall ABI, userspace memory, VFS, tmpfs, IPC, init daemon | 🔜 Next |
| **3 — Horizon** | Shell, desktop, networking, security hardening, package management | 🔭 Future |

See [`docs/kernel/PROGRESS_TRACKER.md`](docs/kernel/PROGRESS_TRACKER.md) for the full execution map and [`all-it-would-take.md`](all-it-would-take.md) for the production milestone breakdown.

## Principles

- **Memory safety** — Rust ownership at every layer, no garbage collector
- **Lean** — One architecture (AArch64), one boot path, no legacy bloat
- **Deterministic** — Explicit boot orchestration, reproducible traces
- **Observable** — Serial output from instruction one, structured fault dumps
- **Tested** — Unit tests, integration tests, QEMU harness, CI gates
- **Built in public** — Every commit visible, no vaporware

## License

MIT

---

Built by Digitwhale.
