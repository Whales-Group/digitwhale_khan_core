# JevKen Production Progress Tracker

Status legend: `[ ]` not started, `[/]` in progress, `[x]` done, `[!]` blocked
Updated: 2026-05-19

## 0) System Type: Layered Orchestration
- [x] Runtime layer registration table
- [x] Immutable system-layer policy
- [x] User-layer creation API
- [x] Runtime enable/disable per layer
- [x] Before/after lifecycle hook execution
- [x] Tests for lock policy + registration + hooks

## 1) Bootloader / Entry Point
- [x] `no_std` kernel crate
- [x] `no_main` entry with `_start`
- [x] Minimal stack setup and jump to Rust kernel main
- [x] Boot flow choice finalized (UEFI primary, C-ABI fallback)

## 2) Memory Management
- [x] Frame allocator (bitmap, two init paths)
- [x] AArch64 4-level page tables + identity mapping
- [x] Heap allocator (bump allocator, 256 KiB)
- [x] Page fault handler (FAR/ESR dump)
- [x] GlobalAlloc foundation (kmalloc/kfree API)

## 3) Interrupt Handling
- [x] AArch64 exception vector table (VBAR_EL1, 2048-byte aligned)
- [x] CPU exception handlers (sync/irq/fiq/serror — dump + halt)
- [x] GICv2 init (distributor + CPU interface, QEMU virt)

## 4) Hardware Abstraction / Drivers
- [x] Serial output (PL011 UART with ring buffer)
- [x] Framebuffer module (clear, put_pixel, fill_rect, put_char, write_str, GOP capture)
- [x] Keyboard input module (PL050 KMI, scancode decode, ring buffer, blocking read_line)
- [x] PCI enumeration (ECAM scan, vendor/device/class readout)

## 5) Process / Task Management
- [x] Task state model (Ready/Running/Blocked/Exited, Task struct with context, stack, pid)
- [x] Context switch save/restore (x0-x30, sp, elr, spsr, tpidr via assembly)
- [x] Round-robin scheduler (64-task capacity, timeslice tick, spawn/exit/block/wake)
- [x] Privilege separation (EL0 vs EL1 SPSR setup, user/kernel task creation)
- [x] System call instruction path wired (SVC → sync exception → dispatch by number)

## 6) ELF Loader & Init Process
- [x] ELF parser (header validation, program headers, segment loading)
- [x] Per-task page tables (user-space isolation, kernel mapping inheritance)
- [x] Page table switching during context switch
- [x] Init process (pid 1, EL1 task, blocks between ticks)
- [x] Scheduler starts init after boot sequence
- [x] Context switch assembly fixed (x1 base pointer preserved until last restore)

## Current Focus
- `ELF loader + init process + scheduler verified in QEMU`
- Next deliverable: external ELF binary loading / VFS layer / PCI fix for GICv3 QEMU
