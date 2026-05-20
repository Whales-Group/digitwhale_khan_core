#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const SERIAL_BUF_CAP: usize = 2048;

static SERIAL_READY: AtomicBool = AtomicBool::new(false);
static SERIAL_LEN: AtomicUsize = AtomicUsize::new(0);
static mut SERIAL_BUF: [u8; SERIAL_BUF_CAP] = [0; SERIAL_BUF_CAP];

const PL011_BASE: usize = 0x900_0000;

fn pl011_write_byte(byte: u8) {
    let dr = PL011_BASE as *mut u32;
    let fr = (PL011_BASE + 0x018) as *const u32;

    while unsafe { fr.read_volatile() } & (1 << 5) != 0 {
        core::hint::spin_loop();
    }

    unsafe {
        dr.write_volatile(byte as u32);
    }
}

fn pl011_init_hardware() {
    let cr = (PL011_BASE + 0x030) as *mut u32;
    let lcr_h = (PL011_BASE + 0x02C) as *mut u32;

    unsafe {
        cr.write_volatile(0);
        lcr_h.write_volatile(0x70);
        cr.write_volatile(0x301);
    }
}

pub fn init() {
    SERIAL_READY.store(true, Ordering::SeqCst);
    SERIAL_LEN.store(0, Ordering::SeqCst);
    #[cfg(target_os = "uefi")]
    pl011_init_hardware();
}

pub fn write_str(text: &str) {
    if !SERIAL_READY.load(Ordering::SeqCst) {
        return;
    }

    for &b in text.as_bytes() {
        let idx = SERIAL_LEN.fetch_add(1, Ordering::SeqCst);
        if idx < SERIAL_BUF_CAP {
            unsafe {
                SERIAL_BUF[idx] = b;
            }
        }
        #[cfg(target_os = "uefi")]
        pl011_write_byte(b);
    }
}

pub fn write_u64_hex(val: u64) {
    let mut leading = true;
    let mut shift: i32 = 60;
    while shift >= 0 {
        let nibble = ((val >> shift as u64) & 0xF) as u8;
        if leading {
            if nibble != 0 || shift == 0 {
                leading = false;
                write_hex_nibble(nibble);
            }
        } else {
            write_hex_nibble(nibble);
        }
        shift -= 4;
    }
    if leading {
        write_str("0");
    }
}

pub fn write_u64_dec(val: u64) {
    if val == 0 {
        write_str("0");
        return;
    }
    // Buffer digits in reverse
    let mut buf = [0u8; 20];
    let mut n = val;
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    let mut j = i;
    while j > 0 {
        j -= 1;
        write_byte(buf[j]);
    }
}

fn write_hex_nibble(nibble: u8) {
    let c = if nibble < 10 {
        b'0' + nibble
    } else {
        b'a' + nibble - 10
    };
    write_byte(c);
}

fn write_byte(byte: u8) {
    let idx = SERIAL_LEN.fetch_add(1, Ordering::SeqCst);
    if idx < SERIAL_BUF_CAP {
        unsafe {
            SERIAL_BUF[idx] = byte;
        }
    }
    #[cfg(target_os = "uefi")]
    pl011_write_byte(byte);
}

pub fn buffer_len() -> usize {
    SERIAL_LEN.load(Ordering::SeqCst).min(SERIAL_BUF_CAP)
}

pub fn read_byte() -> Option<u8> {
    #[cfg(target_os = "uefi")]
    {
        let fr = (PL011_BASE + 0x018) as *const u32;
        if unsafe { fr.read_volatile() } & (1 << 4) == 0 {
            let dr = PL011_BASE as *const u32;
            let byte = unsafe { dr.read_volatile() } as u8;
            return Some(byte);
        }
    }
    None
}

pub fn buffer_at(index: usize) -> Option<u8> {
    if index >= buffer_len() {
        return None;
    }
    Some(unsafe { SERIAL_BUF[index] })
}
