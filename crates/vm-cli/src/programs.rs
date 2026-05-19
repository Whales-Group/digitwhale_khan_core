use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use vm_core::{BytecodeFile, Instruction, Program};

pub fn load_program(path: Option<PathBuf>) -> Result<Program> {
    match path {
        Some(p) => {
            let bytes = fs::read(p)?;
            Ok(BytecodeFile::decode(&bytes)?)
        }
        None => Ok(sample_program()),
    }
}

pub fn sample_program() -> Program {
    Program::new(vec![
        Instruction::LoadImm(40),
        Instruction::Store(0),
        Instruction::LoadImm(2),
        Instruction::AddMem(0),
        Instruction::Syscall(1),
        Instruction::Print,
        Instruction::Halt,
    ])
}
