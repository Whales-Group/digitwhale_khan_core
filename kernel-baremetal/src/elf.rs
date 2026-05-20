#![allow(dead_code)]

use crate::drivers::serial;
use crate::memory::paging::PAGE_SIZE;

const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;

const EM_AARCH64: u16 = 0xB7;

const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;

const PT_LOAD: u32 = 1;

const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;

#[repr(C)]
struct ElfHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[derive(Debug)]
pub struct LoadedImage {
    pub entry: u64,
    pub base_vaddr: u64,
    pub top_vaddr: u64,
}

pub fn parse_and_load(data: &[u8], page_table: usize) -> Option<LoadedImage> {
    if data.len() < core::mem::size_of::<ElfHeader>() {
        serial::write_str("[elf] too small for header\n");
        return None;
    }

    let header = unsafe { &*(data.as_ptr() as *const ElfHeader) };

    if header.e_ident[0..4] != ELF_MAGIC {
        serial::write_str("[elf] bad magic\n");
        return None;
    }

    if header.e_ident[4] != ELFCLASS64 {
        serial::write_str("[elf] not 64-bit\n");
        return None;
    }

    if header.e_ident[5] != ELFDATA2LSB {
        serial::write_str("[elf] not little-endian\n");
        return None;
    }

    if header.e_machine != EM_AARCH64 {
        serial::write_str("[elf] not aarch64\n");
        return None;
    }

    if header.e_type != ET_EXEC && header.e_type != ET_DYN {
        serial::write_str("[elf] not exec or dyn\n");
        return None;
    }

    let phoff = header.e_phoff as usize;
    let phentsize = header.e_phentsize as usize;
    let phnum = header.e_phnum as usize;

    if phoff + phentsize * phnum > data.len() {
        serial::write_str("[elf] program headers out of bounds\n");
        return None;
    }

    serial::write_str("[elf] loading ");
    serial::write_u64_dec(phnum as u64);
    serial::write_str(" segments\n");

    let mut base_vaddr = u64::MAX;
    let mut top_vaddr = 0u64;

    let mut i = 0;
    while i < phnum {
        let ph_ptr = unsafe { data.as_ptr().add(phoff + i * phentsize) };
        let ph = unsafe { &*(ph_ptr as *const ProgramHeader) };

        if ph.p_type != PT_LOAD {
            i += 1;
            continue;
        }

        let vaddr = ph.p_vaddr;
        let _filesz = ph.p_filesz as usize;
        let memsz = ph.p_memsz as usize;
        let offset = ph.p_offset as usize;

        if vaddr + memsz as u64 > 0x0000_FFFF_FFFF_0000 {
            serial::write_str("[elf] vaddr out of user range\n");
            return None;
        }

        if vaddr < base_vaddr {
            base_vaddr = vaddr;
        }
        if vaddr + memsz as u64 > top_vaddr {
            top_vaddr = vaddr + memsz as u64;
        }

        let vaddr_start = vaddr as usize;
        let vaddr_end = (vaddr + memsz as u64) as usize;

        let page_start = align_down(vaddr_start, PAGE_SIZE);
        let page_end = align_up(vaddr_end, PAGE_SIZE);

        let mut addr = page_start;
        while addr < page_end {
            let frame = crate::memory::frame::allocate_frame()?;
            let flags = if ph.p_flags & PF_X != 0 {
                0
            } else {
                1
            };
            crate::memory::paging::map_page_to_table(page_table, addr, frame, flags);

            let page_offset = addr - page_start;
            let copy_start = offset + page_offset;
            let copy_len = if addr + PAGE_SIZE <= vaddr_end {
                PAGE_SIZE
            } else {
                vaddr_end - addr
            };

            let file_data_start = if copy_start < vaddr_start {
                vaddr_start - copy_start
            } else {
                0
            };

            let actual_copy = if file_data_start < copy_len {
                copy_len - file_data_start
            } else {
                0
            };

            if actual_copy > 0 && copy_start + file_data_start < data.len() {
                let src = unsafe { data.as_ptr().add(copy_start + file_data_start) };
                let dst = (frame + file_data_start) as *mut u8;
                unsafe {
                    core::ptr::copy_nonoverlapping(src, dst, actual_copy);
                }
            }

            addr += PAGE_SIZE;
        }

        i += 1;
    }

    if base_vaddr == u64::MAX {
        serial::write_str("[elf] no loadable segments\n");
        return None;
    }

    let entry = header.e_entry;

    serial::write_str("[elf] entry=0x");
    serial::write_u64_hex(entry);
    serial::write_str(" base=0x");
    serial::write_u64_hex(base_vaddr);
    serial::write_str(" top=0x");
    serial::write_u64_hex(top_vaddr);
    serial::write_str("\n");

    Some(LoadedImage {
        entry,
        base_vaddr,
        top_vaddr,
    })
}

fn align_down(x: usize, align: usize) -> usize {
    x & !(align - 1)
}

fn align_up(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
}
