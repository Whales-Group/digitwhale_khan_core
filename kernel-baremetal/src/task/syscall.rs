use crate::drivers::serial;

pub const SYS_WRITE: u64 = 1;
pub const SYS_EXIT: u64 = 2;
pub const SYS_YIELD: u64 = 3;
pub const SYS_GETPID: u64 = 4;
pub const SYS_READ: u64 = 5;

pub fn init_syscall_interface() {
    serial::write_str("syscall: interface initialized\n");
}

#[cfg(target_os = "uefi")]
pub fn syscall0(num: u64) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            lateout("x0") ret,
        );
    }
    ret
}

#[cfg(target_os = "uefi")]
pub fn syscall1(num: u64, arg0: u64) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            in("x0") arg0,
            lateout("x0") ret,
        );
    }
    ret
}

#[cfg(not(target_os = "uefi"))]
pub fn syscall0(_num: u64) -> u64 { 0 }

#[cfg(not(target_os = "uefi"))]
pub fn syscall1(_num: u64, _arg0: u64) -> u64 { 0 }

pub fn handle_syscall(num: u64, arg0: u64, _arg1: u64, _arg2: u64) -> u64 {
    match num {
        SYS_WRITE => sys_write(arg0),
        SYS_EXIT => sys_exit(),
        SYS_YIELD => sys_yield(),
        SYS_GETPID => sys_getpid(),
        SYS_READ => sys_read(),
        _ => {
            serial::write_str("[syscall] unknown syscall: 0x");
            serial::write_u64_hex(num);
            serial::write_str("\n");
            u64::MAX
        }
    }
}

fn sys_write(ptr: u64) -> u64 {
    if ptr == 0 {
        return u64::MAX;
    }
    let len = unsafe { *(ptr as *const u64) };
    let buf_ptr = unsafe { *((ptr + 8) as *const u64) };
    if buf_ptr == 0 || len == 0 {
        return 0;
    }
    let buf = unsafe { core::slice::from_raw_parts(buf_ptr as *const u8, len as usize) };
    if let Ok(s) = core::str::from_utf8(buf) {
        serial::write_str(s);
    }
    len
}

fn sys_exit() -> u64 {
    super::scheduler::exit_current();
}

fn sys_yield() -> u64 {
    super::scheduler::schedule();
    0
}

fn sys_getpid() -> u64 {
    super::scheduler::current_pid() as u64
}

fn sys_read() -> u64 {
    0
}
