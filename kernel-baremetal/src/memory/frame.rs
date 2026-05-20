#![allow(dead_code)]

use super::memmap::MemoryRegion;

pub const FRAME_SIZE: usize = 4096;

const MAX_FRAMES: usize = 512 * 1024 * 1024 / FRAME_SIZE;
const BITMAP_WORDS: usize = MAX_FRAMES / 64;

static mut BITMAP: [u64; BITMAP_WORDS] = [0; BITMAP_WORDS];
static mut BASE_ADDR: usize = 0;
static mut TOTAL_FRAMES: usize = 0;

const MT_CONVENTIONAL: u32 = 7;
const MT_BOOT_CODE: u32 = 3;
const MT_BOOT_DATA: u32 = 4;
const MT_LOADER_CODE: u32 = 1;
const MT_LOADER_DATA: u32 = 2;
const MT_RUNTIME_CODE: u32 = 5;
const MT_RUNTIME_DATA: u32 = 6;

fn align_down(x: usize, align: usize) -> usize {
    x & !(align - 1)
}

fn align_up(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
}

pub fn init_frame_allocator() {
    unsafe {
        BASE_ADDR = 0;
        TOTAL_FRAMES = 1024;
        let mut i = 0;
        while i < BITMAP_WORDS {
            BITMAP[i] = !0;
            i += 1;
        }
        let mut f = 0;
        while f < 1024 {
            set_allocated(f, false);
            f += 1;
        }
    }
}

pub fn init_from_memory_map(regions: &[MemoryRegion]) {
    if regions.is_empty() {
        init_frame_allocator();
        return;
    }

    let mut ram_base = usize::MAX;
    let mut ram_top = 0;

    let mut i = 0;
    while i < regions.len() {
        let r = &regions[i];
        let ty = r.region_type;
        if ty == MT_CONVENTIONAL || ty == MT_BOOT_CODE || ty == MT_BOOT_DATA {
            let start = r.phys_start as usize;
            let end = r.phys_end as usize;
            if start < ram_base {
                ram_base = start;
            }
            if end > ram_top {
                ram_top = end;
            }
        }
        i += 1;
    }

    if ram_base == usize::MAX {
        init_frame_allocator();
        return;
    }

    unsafe {
        BASE_ADDR = align_down(ram_base, FRAME_SIZE);
        let total_bytes = (align_up(ram_top, FRAME_SIZE) - BASE_ADDR).min(512 * 1024 * 1024);
        TOTAL_FRAMES = total_bytes / FRAME_SIZE;

        let mut w = 0;
        while w < BITMAP_WORDS {
            BITMAP[w] = !0;
            w += 1;
        }

        let mut j = 0;
        while j < regions.len() {
            let r = &regions[j];
            let ty = r.region_type;
            if ty == MT_CONVENTIONAL || ty == MT_BOOT_CODE || ty == MT_BOOT_DATA {
                mark_region_free(r.phys_start as usize, r.phys_end as usize);
            }
            j += 1;
        }
    }
}

pub fn allocate_frame() -> Option<usize> {
    unsafe {
        let mut frame = 0;
        while frame < TOTAL_FRAMES {
            if !is_allocated(frame) {
                set_allocated(frame, true);
                return Some(BASE_ADDR + frame * FRAME_SIZE);
            }
            frame += 1;
        }
    }
    None
}

pub fn free_frame(addr: usize) {
    if let Some(frame) = frame_index(addr) {
        set_allocated(frame, false);
    }
}

pub fn total_frames() -> usize {
    unsafe { TOTAL_FRAMES }
}

pub fn base_address() -> usize {
    unsafe { BASE_ADDR }
}

pub fn used_frames() -> usize {
    unsafe {
        let mut used = 0;
        let mut frame = 0;
        while frame < TOTAL_FRAMES {
            if is_allocated(frame) {
                used += 1;
            }
            frame += 1;
        }
        used
    }
}

fn frame_index(addr: usize) -> Option<usize> {
    unsafe {
        if addr < BASE_ADDR || !addr.is_multiple_of(FRAME_SIZE) {
            return None;
        }
        let idx = (addr - BASE_ADDR) / FRAME_SIZE;
        if idx < TOTAL_FRAMES {
            Some(idx)
        } else {
            None
        }
    }
}

fn mark_region_free(start: usize, end: usize) {
    unsafe {
        let rstart = start.max(BASE_ADDR);
        let rend = end.min(BASE_ADDR + TOTAL_FRAMES * FRAME_SIZE);
        if rstart >= rend {
            return;
        }
        let first = (rstart - BASE_ADDR) / FRAME_SIZE;
        let last = (rend - BASE_ADDR - 1) / FRAME_SIZE;
        let mut f = first;
        while f <= last {
            set_allocated(f, false);
            f += 1;
        }
    }
}

fn is_allocated(frame: usize) -> bool {
    let word = frame / 64;
    let bit = frame % 64;
    unsafe { (BITMAP[word] & (1u64 << bit)) != 0 }
}

fn set_allocated(frame: usize, value: bool) {
    let word = frame / 64;
    let bit = frame % 64;
    unsafe {
        if value {
            BITMAP[word] |= 1u64 << bit;
        } else {
            BITMAP[word] &= !(1u64 << bit);
        }
    }
}
