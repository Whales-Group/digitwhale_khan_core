use crate::{Instruction, Memory};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LayerError {
    #[error("operation blocked by layer '{layer}': {reason}")]
    Blocked { layer: &'static str, reason: String },
}

#[derive(Debug, Clone)]
pub enum LayerEvent {
    BeforeInstruction { pc: usize, instruction: Instruction },
    BeforeMemoryRead { addr: usize },
    BeforeMemoryWrite { addr: usize, value: i64 },
    BeforeSyscall { syscall_id: u16 },
}

pub trait Layer {
    fn name(&self) -> &'static str;

    fn on_event(&mut self, _event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        Ok(())
    }
}
