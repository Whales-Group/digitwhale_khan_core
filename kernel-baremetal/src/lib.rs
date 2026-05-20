#![no_std]

pub mod boot;
pub mod drivers;
pub mod elf;
pub mod init;
pub mod interrupts;
pub mod kernel_log;
pub mod layered_orchestration;
pub mod memory;
pub mod task;

pub const KERNEL_NAME: &str = "JevKen";

pub fn kernel_init() {
    boot::setup_stack();
    if let Ok(orchestrator) = layered_orchestration::default_orchestrator() {
        if orchestrator.run_boot_sequence().is_ok() {
            kernel_log::info("kernel boot sequence complete");
        }
    }
}

#[cfg(test)]
mod lib_tests;
