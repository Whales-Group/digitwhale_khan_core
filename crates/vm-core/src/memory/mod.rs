use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("memory access out of bounds at address {addr} (len={len})")]
    OutOfBounds { addr: usize, len: usize },
}

#[derive(Debug, Clone)]
pub struct Memory {
    data: HashMap<usize, i64>,
    size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: HashMap::new(),
            size,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn read(&self, addr: usize) -> Result<i64, MemoryError> {
        if addr >= self.size {
            return Err(MemoryError::OutOfBounds { addr, len: self.size });
        }
        Ok(*self.data.get(&addr).unwrap_or(&0))
    }

    pub fn write(&mut self, addr: usize, value: i64) -> Result<(), MemoryError> {
        if addr >= self.size {
            return Err(MemoryError::OutOfBounds { addr, len: self.size });
        }
        if value == 0 {
            self.data.remove(&addr); // keep sparse — don't store zeroes
        } else {
            self.data.insert(addr, value);
        }
        Ok(())
    }
}