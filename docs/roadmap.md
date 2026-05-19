# Roadmap (Mac Terminal First)

## Phase A: Stabilize Terminal VM (Now)

Goal:
- reliable mac terminal workflow where VM boots and shell is usable.

Exit criteria:
- `cargo run -p vm-cli` consistently boots
- shell commands are stable (`help`, `run-demo`, `run`, `status`, `layers`, `mem`, `exit`)
- core behavior regression-tested

## Phase B: Core Hardening

- syscall registry + typed handlers
- bytecode decode/encode fixture tests
- layer policy regression tests

## Phase C: Reintroduce Platforms (Future)

After terminal VM is stable, reintroduce one platform at a time:
1. mac native app wrapper
2. Windows/Linux packaging
3. Android Kotlin + Rust JNI
4. iOS Swift + Rust bridge
5. firmware/baremetal path

Rule:
- each reintroduced target must include scripts, tests, and docs before moving to next.
