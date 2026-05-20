#![allow(dead_code)]

pub mod font;
pub mod framebuffer;
pub mod keyboard;
pub mod pci;
pub mod serial;

pub fn init_early_drivers() {
    // serial already initialized during boot
    framebuffer::init();
    // pci::scan(); // skipped: ECAM address varies by QEMU version
    keyboard::init();
}
