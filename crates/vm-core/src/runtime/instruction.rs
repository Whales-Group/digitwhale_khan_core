#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    LoadImm(i64),
    AddImm(i64),
    SubImm(i64),
    Store(usize),
    Load(usize),
    AddMem(usize),
    Jump(usize),
    JumpIfZero(usize),
    Syscall(u16),
    Print,
    Halt,
}
