# JevanKernel (Terminal CLI Phase)

Current phase is intentionally simple:

- one machine project
- creates and runs a VM on macOS
- boots into a terminal kernel shell

## Run

```bash
cargo run -p vm-cli
```

You will get a prompt:

```text
jevan-kernel>
```

## Shell commands

- `help`
- `run-demo`
- `run <file.vmb>`
- `status`
- `layers`
- `mem <addr>`
- `exit`

## Bytecode utility

```bash
cargo run -p vm-cli -- --dump-sample-bytecode sample.vmb
```

## Scope note

All app packaging/platform support is removed for now so we can test core VM behavior quickly. We can reintroduce channels later in stages.
