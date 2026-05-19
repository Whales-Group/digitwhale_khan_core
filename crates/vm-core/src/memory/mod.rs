use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("memory access out of bounds at address {addr} (len={len})")]
    OutOfBounds { addr: usize, len: usize },
}

#[derive(Debug, Clone)]
pub struct Memory {
    cells: Vec<i64>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![0; size],
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn read(&self, addr: usize) -> Result<i64, MemoryError> {
        self.cells
            .get(addr)
            .copied()
            .ok_or(MemoryError::OutOfBounds {
                addr,
                len: self.cells.len(),
            })
    }

    pub fn write(&mut self, addr: usize, value: i64) -> Result<(), MemoryError> {
        let len = self.cells.len();
        let Some(cell) = self.cells.get_mut(addr) else {
            return Err(MemoryError::OutOfBounds { addr, len });
        };
        *cell = value;
        Ok(())
    }
}
