use crate::Instruction;

use super::{read_i64, read_u16, read_u64_as_usize, BytecodeError, Program};

const MAGIC: &[u8; 4] = b"VMBC";
const VERSION: u8 = 1;

pub struct BytecodeFile;

impl BytecodeFile {
    pub fn encode(program: &Program) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(MAGIC);
        bytes.push(VERSION);
        bytes.push(0);
        bytes.extend_from_slice(&(program.len() as u16).to_le_bytes());

        for ins in program.instructions() {
            encode_instruction(ins, &mut bytes);
        }
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Program, BytecodeError> {
        if bytes.len() < 8 {
            return Err(BytecodeError::TooShort);
        }
        if &bytes[0..4] != MAGIC {
            return Err(BytecodeError::InvalidMagic);
        }
        if bytes[4] != VERSION {
            return Err(BytecodeError::UnsupportedVersion(bytes[4]));
        }

        let count = u16::from_le_bytes([bytes[6], bytes[7]]) as usize;
        let mut cursor = 8;
        let mut code = Vec::with_capacity(count);

        for _ in 0..count {
            code.push(decode_instruction(bytes, &mut cursor)?);
        }
        Ok(Program::new(code))
    }
}

fn encode_instruction(ins: &Instruction, bytes: &mut Vec<u8>) {
    match ins {
        Instruction::Nop => bytes.push(0x00),
        Instruction::LoadImm(v) => push_i64(0x01, *v, bytes),
        Instruction::AddImm(v) => push_i64(0x02, *v, bytes),
        Instruction::SubImm(v) => push_i64(0x03, *v, bytes),
        Instruction::Store(a) => push_u64(0x04, *a, bytes),
        Instruction::Load(a) => push_u64(0x05, *a, bytes),
        Instruction::AddMem(a) => push_u64(0x06, *a, bytes),
        Instruction::Jump(t) => push_u64(0x07, *t, bytes),
        Instruction::JumpIfZero(t) => push_u64(0x08, *t, bytes),
        Instruction::Print => bytes.push(0x09),
        Instruction::Halt => bytes.push(0x0A),
        Instruction::Syscall(id) => {
            bytes.push(0x0B);
            bytes.extend_from_slice(&id.to_le_bytes());
        }
    }
}

fn decode_instruction(bytes: &[u8], cursor: &mut usize) -> Result<Instruction, BytecodeError> {
    let opcode = *bytes.get(*cursor).ok_or(BytecodeError::Truncated)?;
    *cursor += 1;

    let ins = match opcode {
        0x00 => Instruction::Nop,
        0x01 => Instruction::LoadImm(read_i64(bytes, cursor)?),
        0x02 => Instruction::AddImm(read_i64(bytes, cursor)?),
        0x03 => Instruction::SubImm(read_i64(bytes, cursor)?),
        0x04 => Instruction::Store(read_u64_as_usize(bytes, cursor)?),
        0x05 => Instruction::Load(read_u64_as_usize(bytes, cursor)?),
        0x06 => Instruction::AddMem(read_u64_as_usize(bytes, cursor)?),
        0x07 => Instruction::Jump(read_u64_as_usize(bytes, cursor)?),
        0x08 => Instruction::JumpIfZero(read_u64_as_usize(bytes, cursor)?),
        0x09 => Instruction::Print,
        0x0A => Instruction::Halt,
        0x0B => Instruction::Syscall(read_u16(bytes, cursor)?),
        other => return Err(BytecodeError::InvalidOpcode(other)),
    };

    Ok(ins)
}

fn push_i64(opcode: u8, value: i64, bytes: &mut Vec<u8>) {
    bytes.push(opcode);
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(opcode: u8, value: usize, bytes: &mut Vec<u8>) {
    bytes.push(opcode);
    bytes.extend_from_slice(&(value as u64).to_le_bytes());
}
