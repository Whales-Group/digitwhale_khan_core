# vm-cli Module Breakdown

Purpose: terminal kernel shell over `vm-core`.

Current responsibilities:
- boot VM with mac profile defaults
- parse CLI args
- launch interactive command loop
- run demo or bytecode programs
- inspect status/layers/memory from shell commands

This crate is the primary user entrypoint in the current phase.
