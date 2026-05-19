use crate::Program;

#[derive(Debug, Clone)]
pub struct VmTask {
    pub name: String,
    pub program: Program,
    pub syscall_args: Vec<String>,
}

impl VmTask {
    pub fn new(name: impl Into<String>, program: Program) -> Self {
        Self {
            name: name.into(),
            program,
            syscall_args: vec![],
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.syscall_args = args;
        self
    }
}
