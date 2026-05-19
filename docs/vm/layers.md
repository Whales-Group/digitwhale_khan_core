# VM Layers

JevanKernel VM policies are composed with `LayerOrchestrator`.

## Built-in layers

- `SecurityLayer`
  - coarse memory ceiling guard.
- `CapabilityLayer`
  - memory-region allowlist.
  - syscall allowlist.
- `PlatformProfileLayer`
  - currently used with mac defaults in CLI boot.

## Current runtime mode

The CLI boots with mac profile defaults only.

```bash
cargo run -p vm-cli
```

## VM syscall event

Instruction: `Syscall(u16)`

When executed, VM emits `LayerEvent::BeforeSyscall { syscall_id }` and policies can allow or block it.
