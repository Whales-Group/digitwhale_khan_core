# vm-core Module Breakdown

`vm-core/src/` is split by subsystem:

- `bytecode/`
  - parse/encode VMBC files.
  - owns program loading contract.
- `memory/`
  - bounded memory abstraction.
  - read/write safety checks.
- `logging/`
  - log levels, event shape, logger trait.
- `layer/`
  - policy layer trait.
  - orchestrator for layer order/enablement.
- `runtime/`
  - VM execution engine.
  - instruction dispatch + integration with memory, layers, logging.

This modular split keeps behavior composable and testable per section.
