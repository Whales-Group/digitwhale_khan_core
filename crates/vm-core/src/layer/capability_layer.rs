use std::collections::BTreeSet;

use crate::Memory;

use super::{Layer, LayerError, LayerEvent};

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: usize,
    pub end_inclusive: usize,
}

impl MemoryRegion {
    pub fn contains(&self, addr: usize) -> bool {
        self.start <= addr && addr <= self.end_inclusive
    }
}

pub struct CapabilityLayer {
    allowed_regions: Vec<MemoryRegion>,
    allowed_syscalls: BTreeSet<u16>,
}

impl CapabilityLayer {
    pub fn new(allowed_regions: Vec<MemoryRegion>, allowed_syscalls: impl IntoIterator<Item = u16>) -> Self {
        Self {
            allowed_regions,
            allowed_syscalls: allowed_syscalls.into_iter().collect(),
        }
    }

    fn is_addr_allowed(&self, addr: usize) -> bool {
        self.allowed_regions.iter().any(|r| r.contains(addr))
    }
}

impl Layer for CapabilityLayer {
    fn name(&self) -> &'static str {
        "capability"
    }

    fn on_event(&mut self, event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        match event {
            LayerEvent::BeforeMemoryRead { addr } | LayerEvent::BeforeMemoryWrite { addr, .. } => {
                if self.is_addr_allowed(*addr) {
                    Ok(())
                } else {
                    Err(LayerError::Blocked {
                        layer: self.name(),
                        reason: format!("memory address {} is outside allowed regions", addr),
                    })
                }
            }
            LayerEvent::BeforeSyscall { syscall_id, .. } => {
                if self.allowed_syscalls.contains(syscall_id) {
                    Ok(())
                } else {
                    Err(LayerError::Blocked {
                        layer: self.name(),
                        reason: format!("syscall {} is not permitted", syscall_id),
                    })
                }
            }
            _ => Ok(()),
        }
    }
}
