# JevanKernel VM Bytecode Format (v1)

## Header

- Magic: `VMBC` (4 bytes)
- Version: `u8` (`1`)
- Reserved: `u8` (`0`)
- Instruction count: `u16` little-endian

## Instruction Encoding

- `0x00` `Nop`
- `0x01` `LoadImm(i64)`
- `0x02` `AddImm(i64)`
- `0x03` `SubImm(i64)`
- `0x04` `Store(addr:u64)`
- `0x05` `Load(addr:u64)`
- `0x06` `AddMem(addr:u64)`
- `0x07` `Jump(target:u64)`
- `0x08` `JumpIfZero(target:u64)`
- `0x09` `Print`
- `0x0A` `Halt`
- `0x0B` `Syscall(id:u16)`

All immediates are little-endian.

## Generate sample bytecode

```bash
cargo run -p vm-cli -- --dump-sample-bytecode sample.vmb
```

## Run from file

```bash
cargo run -p vm-cli -- --bytecode sample.vmb --log-level trace
```
