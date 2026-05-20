#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryRegion {
    pub phys_start: u64,
    pub phys_end: u64,
    pub region_type: u32,
    pub attr: u64,
}

impl MemoryRegion {
    pub const fn zero() -> Self {
        Self {
            phys_start: 0,
            phys_end: 0,
            region_type: 0,
            attr: 0,
        }
    }
}

const MAX_REGIONS: usize = 256;

static mut REGIONS: [MemoryRegion; MAX_REGIONS] = [MemoryRegion::zero(); MAX_REGIONS];
static mut REGION_COUNT: usize = 0;

pub fn set_memory_map(src: &[MemoryRegion]) {
    let count = src.len().min(MAX_REGIONS);
    unsafe {
        REGION_COUNT = count;
        let mut i = 0;
        while i < count {
            REGIONS[i] = src[i];
            i += 1;
        }
    }
}

pub fn memory_map() -> &'static [MemoryRegion] {
    unsafe { &REGIONS[..REGION_COUNT] }
}
