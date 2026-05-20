#![allow(dead_code)]

pub mod entry;
pub mod stack;

pub use entry::kernel_entry;
pub use stack::setup_stack;

pub fn jump_to_kernel_main() -> ! {
    crate::kernel_log::info("entering kernel main loop");
    loop {
        core::hint::spin_loop();
    }
}
