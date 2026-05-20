# Layered Orchestration (JevKen)

`layered_orchestration` is a kernel-native system type for startup and policy ordering.

## Design goals
- no_std friendly
- deterministic boot order
- system layers can be locked
- priorities define execution order

## Current bootstrap order
1. `memory` (priority 10)
2. `interrupts` (priority 20)
3. `drivers` (priority 30)
4. `task` (priority 40)

## Next expansions
- enable/disable flags
- per-layer capability requirements
- runtime user-layer registration
- lifecycle stages (`before/after` hooks)
