#![allow(dead_code)]

#[cfg(target_os = "uefi")]
use core::arch::asm;

#[cfg(target_os = "uefi")]
use super::frame;
#[cfg(target_os = "uefi")]
use super::memmap;

pub const PAGE_SIZE: usize = 4096;

const PTE_VALID: u64 = 1 << 0;
const PTE_TABLE: u64 = 1 << 1;
const PTE_PAGE: u64 = 1 << 1;
const PTE_AF: u64 = 1 << 10;
const PTE_AP_EL1_RW: u64 = 0 << 6;
const PTE_XN: u64 = 1 << 54;
const PTE_PXN: u64 = 1 << 53;
const PTE_ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;

const PAGE_NORMAL: u64 = PTE_VALID | PTE_PAGE | PTE_AF | PTE_AP_EL1_RW | (0 << 2);
const PAGE_DEVICE: u64 = PTE_VALID | PTE_PAGE | PTE_AF | PTE_AP_EL1_RW | (1 << 2) | PTE_XN | PTE_PXN;
const BLOCK_NORMAL: u64 = PTE_VALID | PTE_AF | PTE_AP_EL1_RW | (0 << 2);
const BLOCK_DEVICE: u64 = PTE_VALID | PTE_AF | PTE_AP_EL1_RW | (1 << 2) | PTE_XN | PTE_PXN;
const TABLE_ENTRY: u64 = PTE_VALID | PTE_TABLE;

pub fn init_paging() {
    #[cfg(target_os = "uefi")]
    init_paging_impl();
}

#[cfg(target_os = "uefi")]
fn alloc_table() -> usize {
    frame::allocate_frame().expect("oom: page table frame")
}

#[cfg(target_os = "uefi")]
fn zero_table(paddr: usize) {
    unsafe {
        let ptr = paddr as *mut u64;
        let mut i = 0;
        while i < 512 {
            ptr.add(i).write_volatile(0);
            i += 1;
        }
    }
}

#[cfg(target_os = "uefi")]
fn get_pte(table: usize, index: usize) -> u64 {
    unsafe { ((table as *const u64).add(index)).read_volatile() }
}

#[cfg(target_os = "uefi")]
fn set_pte(table: usize, index: usize, value: u64) {
    unsafe {
        ((table as *mut u64).add(index)).write_volatile(value);
    }
}

#[cfg(target_os = "uefi")]
fn get_or_create_l2(l1: usize, l1_idx: usize) -> usize {
    let entry = get_pte(l1, l1_idx);
    if entry & PTE_VALID != 0 {
        (entry & PTE_ADDR_MASK) as usize
    } else {
        let l2 = alloc_table();
        zero_table(l2);
        set_pte(l1, l1_idx, (l2 as u64 & PTE_ADDR_MASK) | TABLE_ENTRY);
        l2
    }
}

#[cfg(target_os = "uefi")]
fn get_or_create_l3(l2: usize, l2_idx: usize) -> usize {
    let entry = get_pte(l2, l2_idx);
    if entry & PTE_VALID != 0 {
        (entry & PTE_ADDR_MASK) as usize
    } else {
        let l3 = alloc_table();
        zero_table(l3);
        set_pte(l2, l2_idx, (l3 as u64 & PTE_ADDR_MASK) | TABLE_ENTRY);
        l3
    }
}

#[cfg(target_os = "uefi")]
fn map_2mb_block(l1: usize, vaddr: usize, paddr: usize, device: bool) {
    let l1_idx = (vaddr >> 30) & 0x1FF;
    let l2_idx = (vaddr >> 21) & 0x1FF;
    let l2 = get_or_create_l2(l1, l1_idx);
    let flags = if device { BLOCK_DEVICE } else { BLOCK_NORMAL };
    let entry = (paddr as u64 & PTE_ADDR_MASK) | flags;
    set_pte(l2, l2_idx, entry);
}

#[cfg(target_os = "uefi")]
fn map_4k_page(l1: usize, vaddr: usize, paddr: usize, device: bool) {
    let l1_idx = (vaddr >> 30) & 0x1FF;
    let l2_idx = (vaddr >> 21) & 0x1FF;
    let l3_idx = (vaddr >> 12) & 0x1FF;
    let l2 = get_or_create_l2(l1, l1_idx);
    let l3 = get_or_create_l3(l2, l2_idx);
    let flags = if device { PAGE_DEVICE } else { PAGE_NORMAL };
    let entry = (paddr as u64 & PTE_ADDR_MASK) | flags;
    set_pte(l3, l3_idx, entry);
}

#[cfg(target_os = "uefi")]
fn identity_map_range(l1: usize, start: usize, end: usize, device: bool) {
    let mut addr = start;
    while addr < end && addr % (2 * 1024 * 1024) != 0 {
        map_4k_page(l1, addr, addr, device);
        addr += PAGE_SIZE;
    }
    while addr + 2 * 1024 * 1024 <= end {
        map_2mb_block(l1, addr, addr, device);
        addr += 2 * 1024 * 1024;
    }
    while addr < end {
        map_4k_page(l1, addr, addr, device);
        addr += PAGE_SIZE;
    }
}

#[cfg(target_os = "uefi")]
fn is_ram_type(ty: u32) -> bool {
    ty == 1 || ty == 2 || ty == 3 || ty == 4 || ty == 5 || ty == 6 || ty == 7
}

#[cfg(target_os = "uefi")]
fn read_id_aa64mmfr0_el1() -> u64 {
    let val: u64;
    unsafe { asm!("mrs {}, id_aa64mmfr0_el1", out(reg) val); }
    val
}

#[cfg(target_os = "uefi")]
fn ensure_l1(l0: usize, l0_idx: usize) -> usize {
    let entry = get_pte(l0, l0_idx);
    if entry & PTE_VALID != 0 {
        (entry & PTE_ADDR_MASK) as usize
    } else {
        let l1 = alloc_table();
        zero_table(l1);
        set_pte(l0, l0_idx, (l1 as u64 & PTE_ADDR_MASK) | TABLE_ENTRY);
        l1
    }
}

#[cfg(target_os = "uefi")]
fn map_4k_page_4l(l0: usize, vaddr: usize, paddr: usize, device: bool) {
    let l0_idx = (vaddr >> 39) & 0x1FF;
    let l1_idx = (vaddr >> 30) & 0x1FF;
    let l2_idx = (vaddr >> 21) & 0x1FF;
    let l3_idx = (vaddr >> 12) & 0x1FF;
    let l1 = ensure_l1(l0, l0_idx);
    let l2 = get_or_create_l2(l1, l1_idx);
    let l3 = get_or_create_l3(l2, l2_idx);
    let flags = if device { PAGE_DEVICE } else { PAGE_NORMAL };
    let entry = (paddr as u64 & PTE_ADDR_MASK) | flags;
    set_pte(l3, l3_idx, entry);
}

#[cfg(target_os = "uefi")]
fn map_2mb_block_4l(l0: usize, vaddr: usize, paddr: usize, device: bool) {
    let l0_idx = (vaddr >> 39) & 0x1FF;
    let l1_idx = (vaddr >> 30) & 0x1FF;
    let l2_idx = (vaddr >> 21) & 0x1FF;
    let l1 = ensure_l1(l0, l0_idx);
    let l2 = get_or_create_l2(l1, l1_idx);
    let flags = if device { BLOCK_DEVICE } else { BLOCK_NORMAL };
    let entry = (paddr as u64 & PTE_ADDR_MASK) | flags;
    set_pte(l2, l2_idx, entry);
}

#[cfg(target_os = "uefi")]
fn identity_map_range_4l(l0: usize, start: usize, end: usize, device: bool) {
    let mut addr = start;
    while addr < end && addr % (2 * 1024 * 1024) != 0 {
        map_4k_page_4l(l0, addr, addr, device);
        addr += PAGE_SIZE;
    }
    while addr + 2 * 1024 * 1024 <= end {
        map_2mb_block_4l(l0, addr, addr, device);
        addr += 2 * 1024 * 1024;
    }
    while addr < end {
        map_4k_page_4l(l0, addr, addr, device);
        addr += PAGE_SIZE;
    }
}

#[cfg(target_os = "uefi")]
fn init_paging_impl() {
    let regions = memmap::memory_map();

    let l0 = alloc_table();
    zero_table(l0);
    unsafe { ROOT_TABLE = l0; }

    let uart_base = 0x0900_0000_usize;
    let uart_end = uart_base + PAGE_SIZE;
    let mut uart_mapped = false;

    let gicd_base = 0x0800_0000_usize;
    let gicd_end = gicd_base + 0x1_0000;
    let gicc_base = 0x0801_0000_usize;
    let gicc_end = gicc_base + 0x1_0000;
    let kmi_base = 0x0902_0000_usize;
    let kmi_end = kmi_base + PAGE_SIZE;
    let pci_ecam_base = 0x3F00_0000_usize;
    let pci_ecam_end = pci_ecam_base + 0x100_0000;

    let mut i = 0;
    while i < regions.len() {
        let r = &regions[i];
        let start = r.phys_start as usize;
        let end = r.phys_end as usize;
        if is_ram_type(r.region_type) {
            identity_map_range_4l(l0, start, end, false);
        }
        if start <= uart_base && end >= uart_end {
            uart_mapped = true;
        }
        i += 1;
    }

    if !uart_mapped {
        identity_map_range_4l(l0, uart_base, uart_end, true);
    }

    identity_map_range_4l(l0, gicd_base, gicd_end, true);
    identity_map_range_4l(l0, gicc_base, gicc_end, true);
    identity_map_range_4l(l0, kmi_base, kmi_end, true);
    identity_map_range_4l(l0, pci_ecam_base, pci_ecam_end, true);

    unsafe {
        let mair: u64 = (0xFF << 0) | (0x00 << 8);
        asm!("msr mair_el1, {}", in(reg) mair);

        let tcr: u64 = (0b00 << 14)
            | (0b11 << 12)
            | (0b01 << 10)
            | (0b01 << 8)
            | (0 << 7)
            | (16 << 0)
            | (0b10 << 30)
            | (0b11 << 28)
            | (0b01 << 26)
            | (0b01 << 24)
            | (0 << 23)
            | (16 << 16)
            | (2 << 32)
            | (1 << 38);
        asm!("msr tcr_el1, {}", in(reg) tcr);

        asm!("msr ttbr0_el1, {}", in(reg) l0 as u64);
        asm!("dsb sy");
        asm!("isb");
        asm!("tlbi vmalle1");
        asm!("dsb sy");
        asm!("isb");
    }
}

static mut ROOT_TABLE: usize = 0;

pub fn kernel_table() -> usize {
    unsafe { ROOT_TABLE }
}

#[cfg(target_os = "uefi")]
pub fn create_user_table(kernel_table: usize) -> usize {
    let l0 = alloc_table();
    zero_table(l0);

    let mut i = 0;
    while i < 256 {
        let entry = get_pte(kernel_table, i);
        if entry & PTE_VALID != 0 {
            set_pte(l0, i, entry);
        }
        i += 1;
    }

    l0
}

#[cfg(not(target_os = "uefi"))]
pub fn create_user_table(_kernel_table: usize) -> usize {
    0
}

pub fn map_page_to_table(_table: usize, _virt: usize, _phys: usize, _flags: u64) {
    #[cfg(target_os = "uefi")]
    {
        let device = (_flags & 1) != 0;
        if _table == 0 {
            return;
        }
        map_4k_page_4l(_table, _virt, _phys, device);
    }
}

pub fn switch_page_table(_table: usize) {
    #[cfg(target_os = "uefi")]
    unsafe {
        core::arch::asm!("msr ttbr0_el1, {}", in(reg) _table as u64);
        core::arch::asm!("dsb sy");
        core::arch::asm!("isb");
        core::arch::asm!("tlbi vmalle1");
        core::arch::asm!("dsb sy");
        core::arch::asm!("isb");
    }
}

pub fn map_page(_virt: usize, _phys: usize, _flags: u64) {
    #[cfg(target_os = "uefi")]
    {
        let device = (_flags & 1) != 0;
        let l0 = unsafe { ROOT_TABLE };
        if l0 == 0 {
            return;
        }
        map_4k_page_4l(l0, _virt, _phys, device);
    }
}

pub fn handle_page_fault(_addr: usize, _error_code: u64) {
    #[cfg(target_os = "uefi")]
    {
        let far: u64;
        let esr: u64;
        unsafe {
            core::arch::asm!("mrs {}, far_el1", out(reg) far);
            core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
        }
        crate::drivers::serial::write_str("\n[fault] page fault at 0x");
        crate::drivers::serial::write_u64_hex(_addr as u64);
        crate::drivers::serial::write_str(" far=0x");
        crate::drivers::serial::write_u64_hex(far);
        crate::drivers::serial::write_str(" esr=0x");
        crate::drivers::serial::write_u64_hex(esr);
        crate::drivers::serial::write_str("\n");
    }
    loop {
        core::hint::spin_loop();
    }
}
