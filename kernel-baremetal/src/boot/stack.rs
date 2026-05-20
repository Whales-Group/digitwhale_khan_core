#![allow(dead_code)]

pub fn setup_stack() {
    #[cfg(target_os = "uefi")]
    {
        let sp: u64;
        unsafe { core::arch::asm!("mov {}, sp", out(reg) sp); }
        if sp == 0 {
            crate::kernel_log::info("boot: sp is zero on UEFI entry — system unstable");
            loop { core::hint::spin_loop(); }
        }
    }

}
