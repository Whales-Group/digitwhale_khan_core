use core::arch::global_asm;

#[repr(C)]
pub struct ExceptionContext {
    pub x: [u64; 30],
    pub lr: u64,
    pub elr: u64,
    pub spsr: u64,
}

unsafe extern "C" {
    static exception_vector_base: u8;
}

global_asm!(
    ".section .text.vectors, \"ax\"",
    ".balign 2048",
    ".globl exception_vector_base",
    "exception_vector_base:",

    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",

    ".balign 128", "b sync_entry",
    ".balign 128", "b irq_entry",
    ".balign 128", "b fiq_entry",
    ".balign 128", "b serror_entry",

    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",

    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",
    ".balign 128", "b unhandled_entry",

    ".macro save_context",
    "  sub sp, sp, #264",
    "  stp x0, x1, [sp, #0]",
    "  stp x2, x3, [sp, #16]",
    "  stp x4, x5, [sp, #32]",
    "  stp x6, x7, [sp, #48]",
    "  stp x8, x9, [sp, #64]",
    "  stp x10, x11, [sp, #80]",
    "  stp x12, x13, [sp, #96]",
    "  stp x14, x15, [sp, #112]",
    "  stp x16, x17, [sp, #128]",
    "  stp x18, x19, [sp, #144]",
    "  stp x20, x21, [sp, #160]",
    "  stp x22, x23, [sp, #176]",
    "  stp x24, x25, [sp, #192]",
    "  stp x26, x27, [sp, #208]",
    "  stp x28, x29, [sp, #224]",
    "  str x30, [sp, #240]",
    "  mrs x0, elr_el1",
    "  str x0, [sp, #248]",
    "  mrs x0, spsr_el1",
    "  str x0, [sp, #256]",
    ".endm",

    ".macro restore_context",
    "  ldp x0, x1, [sp, #0]",
    "  ldp x2, x3, [sp, #16]",
    "  ldp x4, x5, [sp, #32]",
    "  ldp x6, x7, [sp, #48]",
    "  ldp x8, x9, [sp, #64]",
    "  ldp x10, x11, [sp, #80]",
    "  ldp x12, x13, [sp, #96]",
    "  ldp x14, x15, [sp, #112]",
    "  ldp x16, x17, [sp, #128]",
    "  ldp x18, x19, [sp, #144]",
    "  ldp x20, x21, [sp, #160]",
    "  ldp x22, x23, [sp, #176]",
    "  ldp x24, x25, [sp, #192]",
    "  ldp x26, x27, [sp, #208]",
    "  ldp x28, x29, [sp, #224]",
    "  ldr x30, [sp, #240]",
    "  ldr x0, [sp, #248]",
    "  msr elr_el1, x0",
    "  ldr x0, [sp, #256]",
    "  msr spsr_el1, x0",
    "  add sp, sp, #264",
    "  eret",
    ".endm",

    "unhandled_entry:",
    "  save_context",
    "  mov x0, sp",
    "  bl {unhandled}",
    "  restore_context",

    "sync_entry:",
    "  save_context",
    "  mov x0, sp",
    "  bl {sync}",
    "  restore_context",

    "irq_entry:",
    "  save_context",
    "  mov x0, sp",
    "  bl {irq}",
    "  restore_context",

    "fiq_entry:",
    "  save_context",
    "  mov x0, sp",
    "  bl {fiq}",
    "  restore_context",

    "serror_entry:",
    "  save_context",
    "  mov x0, sp",
    "  bl {serror}",
    "  restore_context",

    unhandled = sym rust_unhandled_exception,
    sync = sym rust_sync_exception,
    irq = sym rust_irq_handler,
    fiq = sym rust_fiq_handler,
    serror = sym rust_serror_handler,
);

pub fn init_exception_vectors() {
    #[cfg(target_os = "uefi")]
    {
        let vbar = unsafe { &exception_vector_base as *const u8 as u64 };
        unsafe {
            core::arch::asm!("msr vbar_el1, {}", in(reg) vbar);
            core::arch::asm!("dsb sy");
            core::arch::asm!("isb");
        }
        crate::kernel_log::info("vectors: VBAR_EL1 installed");
    }
}

fn dump_context(ctx: &ExceptionContext) {
    crate::drivers::serial::write_str("\n  pc=0x");
    crate::drivers::serial::write_u64_hex(ctx.elr);
    crate::drivers::serial::write_str(" spsr=0x");
    crate::drivers::serial::write_u64_hex(ctx.spsr);
    crate::drivers::serial::write_str(" lr=0x");
    crate::drivers::serial::write_u64_hex(ctx.lr);
    crate::drivers::serial::write_str("\n  x0=0x");
    crate::drivers::serial::write_u64_hex(ctx.x[0]);
    crate::drivers::serial::write_str(" x1=0x");
    crate::drivers::serial::write_u64_hex(ctx.x[1]);
    crate::drivers::serial::write_str("\n");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_unhandled_exception(ctx: *mut ExceptionContext) {
    let ctx = unsafe { &*ctx };
    crate::drivers::serial::write_str("\n[panic] unhandled exception at EL1");
    dump_context(ctx);
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_sync_exception(ctx: *mut ExceptionContext) {
    let ctx = unsafe { &mut *ctx };
    let esr: u64;
    let far: u64;
    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
        core::arch::asm!("mrs {}, far_el1", out(reg) far);
    }

    let ec = (esr >> 26) & 0x3F;
    const EC_SVC_AARCH64: u64 = 0b010101;

    if ec == EC_SVC_AARCH64 {
        let imm16 = esr & 0xFFFF;
        let x0 = ctx.x[0];
        let x1 = ctx.x[1];
        let x2 = ctx.x[2];
        let ret = crate::task::handle_syscall(imm16, x0, x1, x2);
        ctx.x[0] = ret;
        ctx.elr += 4;
        return;
    }

    crate::drivers::serial::write_str("\n[fault] sync exception");
    crate::drivers::serial::write_str(" esr=0x");
    crate::drivers::serial::write_u64_hex(esr);
    crate::drivers::serial::write_str(" far=0x");
    crate::drivers::serial::write_u64_hex(far);
    dump_context(ctx);
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_irq_handler(ctx: *mut ExceptionContext) {
    let _ctx = unsafe { &*ctx };
    crate::drivers::keyboard::handle_irq();
    crate::task::tick();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_fiq_handler(ctx: *mut ExceptionContext) {
    let ctx = unsafe { &*ctx };
    crate::drivers::serial::write_str("\n[fiq] fast interrupt");
    dump_context(ctx);
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_serror_handler(ctx: *mut ExceptionContext) {
    let ctx = unsafe { &*ctx };
    crate::drivers::serial::write_str("\n[serror] system error");
    dump_context(ctx);
    loop {
        core::hint::spin_loop();
    }
}
