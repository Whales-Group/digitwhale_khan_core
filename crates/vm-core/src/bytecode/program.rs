use crate::Instruction;

#[derive(Debug, Clone)]
pub struct Program {
    code: Vec<Instruction>,
}

impl Program {
    pub fn new(code: Vec<Instruction>) -> Self {
        Self { code }
    }

    pub fn get(&self, pc: usize) -> Option<&Instruction> {
        self.code.get(pc)
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.code
    }
}
