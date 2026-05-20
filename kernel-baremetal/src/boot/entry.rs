use crate::kernel_init;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_entry() -> ! {
    #[cfg(all(not(target_os = "uefi"), not(test)))]
    {
        unsafe extern "C" {
            static _kernel_stack_end: u8;
        }
        let sp = unsafe { &_kernel_stack_end as *const u8 as u64 };
        unsafe {
            core::arch::asm!("mov sp, {}", in(reg) sp);
            core::arch::asm!("dsb sy");
            core::arch::asm!("isb");
        }
    }
    kernel_init();
    loop {
        core::hint::spin_loop();
    }
}
