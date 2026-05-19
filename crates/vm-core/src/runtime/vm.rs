use crate::{LayerOrchestrator, Memory, MemoryError};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub pc: usize,
    pub acc: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct VmConfig {
    pub memory_size: usize,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self { memory_size: 256 }
    }
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("program counter out of bounds at {0}")]
    PcOutOfBounds(usize),
    #[error("memory error: {0}")]
    Memory(#[from] MemoryError),
    #[error("layer error: {0}")]
    Layer(#[from] crate::LayerError),
}

pub struct Vm {
    pub(crate) regs: Registers,
    pub(crate) halted: bool,
    pub(crate) output: Vec<String>,
    pub(crate) memory: Memory,
    pub(crate) layers: LayerOrchestrator,
    pub(crate) syscall_args: Vec<String>,
}

impl Vm {
    pub fn new(config: VmConfig) -> Self {
        Self {
            regs: Registers { pc: 0, acc: 0 },
            halted: false,
            output: vec![],
            memory: Memory::new(config.memory_size),
            layers: LayerOrchestrator::default(),
            syscall_args: vec![],
        }
    }

    pub fn set_syscall_args(&mut self, args: Vec<String>) { self.syscall_args = args; }
    pub fn registers(&self) -> Registers { self.regs }
    pub fn output(&self) -> &[String] { &self.output }
    pub fn memory(&self) -> &Memory { &self.memory }
    pub fn layers_mut(&mut self) -> &mut LayerOrchestrator { &mut self.layers }

    pub fn reset(&mut self) {
        self.regs = Registers { pc: 0, acc: 0 };
        self.halted = false;
        self.output.clear();
        self.syscall_args.clear();
    }
}

impl Default for Vm {
    fn default() -> Self { Self::new(VmConfig::default()) }
}
