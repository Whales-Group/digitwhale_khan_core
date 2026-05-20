use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::drivers::serial;

const KBD_BUF_CAP: usize = 256;

static KBD_READY: AtomicBool = AtomicBool::new(false);
static KBD_HEAD: AtomicUsize = AtomicUsize::new(0);
static KBD_TAIL: AtomicUsize = AtomicUsize::new(0);
static mut KBD_BUF: [u8; KBD_BUF_CAP] = [0; KBD_BUF_CAP];

const PL050_KMI_BASE: usize = 0x0902_0000;

const KMICR: usize = 0x00;
const KMISTAT: usize = 0x04;
const KMIDATA: usize = 0x08;

const KMICR_EN: u32 = 1 << 6;
const KMICR_RXINT_EN: u32 = 1 << 4;
const KMICR_TXINT_EN: u32 = 1 << 3;
const KMICR_KMI_EN: u32 = 1 << 0;

const KMISTAT_RXBUSY: u32 = 1 << 2;
const KMISTAT_RXFULL: u32 = 1 << 0;

fn kmi_write32(offset: usize, val: u32) {
    unsafe {
        ((PL050_KMI_BASE + offset) as *mut u32).write_volatile(val);
    }
}

fn kmi_read32(offset: usize) -> u32 {
    unsafe {
        ((PL050_KMI_BASE + offset) as *const u32).read_volatile()
    }
}

fn kmi_read_data() -> Option<u8> {
    if kmi_read32(KMISTAT) & KMISTAT_RXFULL != 0 {
        Some(kmi_read32(KMIDATA) as u8)
    } else {
        None
    }
}

fn push_byte(b: u8) {
    let tail = KBD_TAIL.load(Ordering::SeqCst);
    let head = KBD_HEAD.load(Ordering::SeqCst);
    let next_tail = (tail + 1) % KBD_BUF_CAP;
    if next_tail != head {
        unsafe {
            KBD_BUF[tail] = b;
        }
        KBD_TAIL.store(next_tail, Ordering::SeqCst);
    }
}

fn pop_byte() -> Option<u8> {
    let head = KBD_HEAD.load(Ordering::SeqCst);
    let tail = KBD_TAIL.load(Ordering::SeqCst);
    if head == tail {
        None
    } else {
        let b = unsafe { KBD_BUF[head] };
        KBD_HEAD.store((head + 1) % KBD_BUF_CAP, Ordering::SeqCst);
        Some(b)
    }
}

static mut PREV_SCANCODE: u8 = 0;

fn decode_scancode(sc: u8) -> Option<u8> {
    if sc == 0xF0 {
        unsafe { PREV_SCANCODE = 0xF0; }
        return None;
    }

    let make = unsafe {
        if PREV_SCANCODE == 0xF0 {
            PREV_SCANCODE = 0;
            false
        } else {
            true
        }
    };

    if !make {
        return None;
    }

    Some(match sc {
        0x1C => b'a', 0x32 => b'b', 0x21 => b'c', 0x23 => b'd',
        0x24 => b'e', 0x2B => b'f', 0x34 => b'g', 0x33 => b'h',
        0x43 => b'i', 0x3B => b'j', 0x42 => b'k', 0x4B => b'l',
        0x3A => b'm', 0x31 => b'n', 0x44 => b'o', 0x4D => b'p',
        0x15 => b'q', 0x2D => b'r', 0x1B => b's', 0x2C => b't',
        0x3C => b'u', 0x2A => b'v', 0x1D => b'w', 0x22 => b'x',
        0x35 => b'y', 0x1A => b'z',
        0x45 => b'0', 0x16 => b'1', 0x1E => b'2', 0x26 => b'3',
        0x25 => b'4', 0x2E => b'5', 0x36 => b'6', 0x3D => b'7',
        0x3E => b'8', 0x46 => b'9',
        0x29 => b' ',
        0x5A => b'\n',
        0x66 => b'\x08',
        0x0E => b'`', 0x4E => b'-', 0x55 => b'=',
        0x54 => b'[', 0x5B => b']', 0x5D => b'\\',
        0x4C => b';', 0x52 => b'\'', 0x41 => b',',
        0x49 => b'.', 0x4A => b'/',
        _ => return None,
    })
}

pub fn init() {
    #[cfg(target_os = "uefi")]
    {
        kmi_write32(KMICR, 0);
        kmi_write32(KMICR, KMICR_EN | KMICR_RXINT_EN);
        serial::write_str("keyboard: PL050 KMI initialized\n");
    }
    KBD_READY.store(true, Ordering::SeqCst);
    KBD_HEAD.store(0, Ordering::SeqCst);
    KBD_TAIL.store(0, Ordering::SeqCst);
}

pub fn handle_irq() {
    #[cfg(target_os = "uefi")]
    {
        while let Some(sc) = kmi_read_data() {
            if let Some(c) = decode_scancode(sc) {
                push_byte(c);
            }
        }
    }
}

pub fn read_byte() -> Option<u8> {
    if !KBD_READY.load(Ordering::SeqCst) {
        return None;
    }
    pop_byte()
}

pub fn peek_byte() -> Option<u8> {
    if !KBD_READY.load(Ordering::SeqCst) {
        return None;
    }
    let head = KBD_HEAD.load(Ordering::SeqCst);
    let tail = KBD_TAIL.load(Ordering::SeqCst);
    if head == tail {
        None
    } else {
        Some(unsafe { KBD_BUF[head] })
    }
}

pub fn buffer_len() -> usize {
    let tail = KBD_TAIL.load(Ordering::SeqCst);
    let head = KBD_HEAD.load(Ordering::SeqCst);
    if tail >= head {
        tail - head
    } else {
        KBD_BUF_CAP - head + tail
    }
}

pub fn read_line(buf: &mut [u8]) -> Option<usize> {
    if !KBD_READY.load(Ordering::SeqCst) {
        return None;
    }
    let mut len = 0;
    while len < buf.len() {
        if let Some(b) = read_byte() {
            if b == b'\n' || b == b'\r' {
                if len > 0 {
                    return Some(len);
                }
            } else if b == b'\x08' {
                if len > 0 {
                    len -= 1;
                    serial::write_str("\x08 \x08");
                }
            } else {
                buf[len] = b;
                len += 1;
                let ch = [b];
                if let Ok(s) = core::str::from_utf8(&ch) {
                    serial::write_str(s);
                }
            }
        } else {
            core::hint::spin_loop();
        }
    }
    if len > 0 {
        Some(len)
    } else {
        None
    }
}
