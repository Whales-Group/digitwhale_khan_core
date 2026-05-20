#![allow(dead_code)]

pub mod frame;
#[cfg(test)]
mod frame_tests;
pub mod heap;
pub mod memmap;
pub mod paging;

pub fn init_memory() {
    let regions = memmap::memory_map();
    frame::init_from_memory_map(regions);
    paging::init_paging();
    heap::init_heap();

    let base = frame::base_address();
    let total = frame::total_frames();
    let free = total - frame::used_frames();
    let top = base + total * frame::FRAME_SIZE;

    crate::kernel_log::info("memory init ok");
    crate::drivers::serial::write_str("  phys 0x");
    crate::drivers::serial::write_u64_hex(base as u64);
    crate::drivers::serial::write_str(" - 0x");
    crate::drivers::serial::write_u64_hex(top as u64);
    crate::drivers::serial::write_str("  total=");
    crate::drivers::serial::write_u64_dec(total as u64);
    crate::drivers::serial::write_str("  free=");
    crate::drivers::serial::write_u64_dec(free as u64);
    crate::drivers::serial::write_str("\n");
}
