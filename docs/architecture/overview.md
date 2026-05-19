# JevanKernel Architecture Notes

## Objective

Build a terminal-first VM runtime on macOS, stabilize core behavior, then expand later.

## Current Split

- `vm-core`
  - bytecode runtime semantics
  - memory model
  - logging and telemetry points
  - layer-based policy hooks

- `vm-cli`
  - terminal kernel shell
  - command loop for running programs and inspecting VM state

## Layered Control Plane

`LayerOrchestrator` composes policies in deterministic order:
- add layer with priority
- enable/disable layer
- reorder by priority

This keeps policy modular without hardwiring it into instruction execution.
