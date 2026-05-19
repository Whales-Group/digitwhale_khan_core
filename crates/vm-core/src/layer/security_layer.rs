use crate::Memory;

use super::{Layer, LayerError, LayerEvent};

pub struct SecurityLayer {
    max_memory_addr: usize,
}

impl SecurityLayer {
    pub fn new(max_memory_addr: usize) -> Self {
        Self { max_memory_addr }
    }
}

impl Layer for SecurityLayer {
    fn name(&self) -> &'static str {
        "security"
    }

    fn on_event(&mut self, event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        match event {
            LayerEvent::BeforeMemoryRead { addr } | LayerEvent::BeforeMemoryWrite { addr, .. }
                if *addr > self.max_memory_addr =>
            {
                Err(LayerError::Blocked {
                    layer: self.name(),
                    reason: format!("address {} exceeds allowed maximum {}", addr, self.max_memory_addr),
                })
            }
            _ => Ok(()),
        }
    }
}
