# JevKen Kernel Progress Tracker

Status: `[ ]` not started, `[/]` in progress, `[x]` done, `[!]` blocked
Updated: 2026-05-19

## 0) System Type: Layered Orchestration
- [x] Create `layered_orchestration` kernel module
- [x] Deterministic priority ordering
- [x] Locked system-layer registration
- [x] Runtime layer registration table
- [x] Immutable system-layer policy
- [x] User-layer creation API
- [x] Runtime enable/disable per layer
- [x] Before/after lifecycle hooks
- [x] Tests for policy + registry + hooks

## 1) Bootloader / Entry Point
- [x] Establish `no_std` kernel crate
- [x] Add `no_main` entry with `_start`
- [x] Minimal stack setup and jump to Rust kernel main
- [x] Boot flow choice finalized (UEFI primary, kernel_entry C-ABI fallback)

## 2) Memory Management
- [x] Frame allocator (bitmap, two init paths, contiguous allocation)
- [x] AArch64 4-level page tables (L0-L3, identity map, MAIR/TCR/TTBR)
- [x] Heap allocator (bump, 256 KiB from frame allocator)
- [x] Page fault handler (FAR_EL1 + ESR_EL1 dump)
- [x] kmalloc/kfree API (GlobalAlloc-ready)

## 3) Interrupt Handling
- [x] AArch64 exception vector table (VBAR_EL1, 2048-byte aligned, 16 entries)
- [x] CPU exception handlers (sync/irq/fiq/serror — context save, ESR/FAR dump, halt)
- [x] GICv2 init (distributor enable, interrupt routing, CPU interface)

## 4) Hardware Abstraction / Drivers
- [x] Serial output (PL011 UART with ring buffer)
- [/] Framebuffer module scaffolded
- [/] Keyboard input module scaffolded
- [/] PCI module scaffolded
- [ ] Framebuffer output usable
- [ ] Keyboard input path usable
- [ ] PCI enumeration usable

## 5) Process / Task Management
- [/] Task state model scaffolded
- [ ] Context switch save/restore implementation
- [ ] Round-robin scheduler working
- [ ] Privilege separation (kernel/user)
- [ ] System call instruction path wired

## Current Focus
- `Interrupt handling complete — drivers + tasking next`
- Next deliverable: framebuffer driver / scheduler
