# JevKen: All It Would Take

This is the full execution map from current state to a production-grade kernel.

## 0. Definition of Done (Production)
A production milestone means:
- boots reliably on target hardware/VMs
- survives faults without silent corruption
- has memory/process/security isolation
- has deterministic test coverage + CI gates
- has observable logs/traces/crash dumps
- has upgrade/version/ABI policy
- has docs sufficient for new contributors

## 1. Boot and Platform Bring-Up
### Build/Boot
- finalize boot path choice (UEFI first, then optional GRUB)
- stable linker script + memory map contract
- `_start` and early stack/bootstrap finalized
- architecture-specific entry modules (`x86_64`, `aarch64`)

### Early runtime
- early serial output on real hardware path
- panic path with minimal crash context output
- CPU feature probing and compatibility checks

Acceptance criteria:
- boots to kernel banner in QEMU x86_64 and aarch64
- deterministic boot logs across runs

## 2. Memory Management
### Physical memory manager
- parse firmware memory map
- usable-region validation and reservation
- frame allocator (bitmap/buddy) with fragmentation strategy

### Virtual memory
- page table creation and ownership model
- kernel higher-half mapping strategy
- per-process address spaces
- page fault classification + handlers

### Heap
- `GlobalAlloc` integration with failure behavior
- slab/arena strategy for kernel objects

Acceptance criteria:
- memory stress tests pass
- fault injection does not corrupt allocator state

## 3. Interrupts, Exceptions, Timers
- IDT/GDT/TSS complete and validated
- exception handlers (PF/GP/DF/UD/etc.)
- PIC/APIC or GIC setup per architecture
- timer interrupts for preemption/scheduling

Acceptance criteria:
- exception tests produce controlled kernel behavior
- timer tick stable under load

## 4. Kernel Core Services
- synchronization primitives (`spinlock`, `rwlock`, atomics discipline)
- kernel object lifecycle model
- capability and permission model finalized
- IPC primitives (message queues or channels)

Acceptance criteria:
- lock-order policy documented and linted in review
- no deadlocks in stress suite

## 5. Process and Task Management
- task struct with full CPU context
- scheduler (round-robin -> priority/latency aware)
- sleep/block/wakeup queues
- process creation/termination/wait semantics
- kernel/user privilege transition

Acceptance criteria:
- multitask stress tests pass for long duration
- starvation and priority inversion mitigations tested

## 6. Syscall ABI and Kernel ABI Governance
- freeze syscall interface versions
- argument validation + copy-in/copy-out safety
- compatibility policy (`v0.x`, `v1`) and deprecation model
- ABI conformance tests in CI

Acceptance criteria:
- syscall ABI tests prevent drift
- versioned docs published per release

## 7. Filesystem and Storage
- VFS architecture (mount table, inode/file abstractions)
- minimal filesystem implementation (tmpfs first, then disk-backed)
- block device layer
- caching and writeback strategy

Acceptance criteria:
- fs consistency checks pass after fault/restart tests

## 8. Drivers and HAL
- HAL traits frozen (`time`, `console`, `memory_map`, `irq`, `block`, `net`)
- serial/UART real output path
- framebuffer or text console driver
- keyboard input path
- PCI discovery + driver binding

Acceptance criteria:
- device enumeration works on target VM and reference hardware

## 9. Security Model
- kernel/user separation enforced
- capability checks in all privileged paths
- secure defaults (deny-by-default)
- audit trail hooks

Acceptance criteria:
- privilege escalation test suite passes
- unsafe blocks reviewed with justification docs

## 10. Observability and Diagnostics
- structured kernel event log format
- trace buffers and event categories
- deterministic replay metadata where feasible
- crash dumps with symbolizable addresses

Acceptance criteria:
- postmortem workflow documented and exercised

## 11. Reliability and Quality Gates
### Testing layers
- unit tests (core algorithms)
- integration tests (subsystem interactions)
- property/fuzz tests (parsers, memory logic)
- soak/stress tests (long-running)

### CI/CD
- no-merge-on-red rule
- required checks: format, lint, test, docs, ABI checks
- reproducible builds

Acceptance criteria:
- green pipeline required for merge
- nightly stress report generated

## 12. Tooling and Developer Experience
- clear workspace layout and module ownership
- coding standards (unsafe policy, review checklist)
- scripts for run/test/debug targets
- contributor onboarding guide

Acceptance criteria:
- new contributor can build + run tests in <30 min

## 13. Release Engineering
- semantic versioning for kernel and ABI
- release notes and migration notes
- artifact generation for target platforms
- rollback strategy

Acceptance criteria:
- release candidate checklist fully automated where possible

## 14. Documentation Set Required
- architecture overview
- boot flow spec
- memory model spec
- syscall ABI spec
- layering/orchestration spec
- security model spec
- debugging and incident response guide

Acceptance criteria:
- docs and code stay version-synced

## 15. Organization and Execution Discipline
- define module owners
- weekly milestone reviews
- issue labels by subsystem/risk
- risk register and mitigation tracking

Acceptance criteria:
- every critical subsystem has owner + SLA

## 16. Suggested Execution Order (Practical)
1. boot stabilization + real serial output
2. memory map + frame allocator + paging
3. exceptions/interrupts/timer
4. process context switch + scheduler
5. syscall ABI freeze + user mode boundary
6. VFS + tmpfs + block layer
7. security hardening + observability
8. CI hard gates + release process

## 17. Biggest Risks
- undefined behavior from unsafe memory/interrupt code
- architecture drift without ABI governance
- overbuilding features before core isolation works
- weak observability slowing bug resolution

Mitigation:
- ship subsystem-by-subsystem with hard tests and invariants

## 18. Immediate Next 7 Concrete Tasks
1. implement real serial port output path (not buffer-only)
2. add firmware memory map ingestion contract
3. implement frame allocator reservations for kernel/boot regions
4. implement initial page table setup
5. wire page fault handler with error classification
6. add scheduler tick source from timer interrupt
7. freeze syscall ABI v0.1 and add drift tests
