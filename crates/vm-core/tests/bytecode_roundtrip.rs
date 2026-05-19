use vm_core::{BytecodeFile, Instruction, Program};

#[test]
fn roundtrip_preserves_instruction_count() {
    let program = Program::new(vec![
        Instruction::LoadImm(1),
        Instruction::AddImm(2),
        Instruction::Syscall(10),
        Instruction::Halt,
    ]);

    let bytes = BytecodeFile::encode(&program);
    let decoded = BytecodeFile::decode(&bytes).expect("decode should succeed");

    assert_eq!(program.len(), decoded.len());
}
