use core::sync::atomic::{AtomicU32, Ordering};

use super::context::TaskContext;

static NEXT_PID: AtomicU32 = AtomicU32::new(1);

pub const KERNEL_STACK_SIZE: usize = 8192;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Exited,
}

#[repr(C)]
pub struct Task {
    pub pid: u32,
    pub state: TaskState,
    pub priority: u8,
    pub ticks: u32,
    pub context: TaskContext,
    pub kernel_stack: [u8; KERNEL_STACK_SIZE],
    pub kernel_sp: u64,
    pub page_table: usize,
}

impl Task {
    pub fn new(entry: u64, priority: u8, el0: bool) -> Self {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);

        let stack = [0u8; KERNEL_STACK_SIZE];
        let stack_top = stack.as_ptr() as u64 + KERNEL_STACK_SIZE as u64;

        let spsr = if el0 {
            spsr_el0()
        } else {
            spsr_el1()
        };

        let kernel_table = crate::memory::paging::kernel_table();
        let page_table = if el0 {
            crate::memory::paging::create_user_table(kernel_table)
        } else {
            kernel_table
        };

        let context = TaskContext {
            x: [0; 31],
            sp: stack_top,
            elr: entry,
            spsr,
            tpidr: 0,
        };

        let mut task = Self {
            pid,
            state: TaskState::Ready,
            priority,
            ticks: 0,
            context,
            kernel_stack: stack,
            kernel_sp: stack_top,
            page_table,
        };

        task.context.x[0] = pid as u64;
        task.context.x[30] = entry;

        task
    }

    pub fn idle() -> Self {
        let pid = 0;
        let kernel_table = crate::memory::paging::kernel_table();
        Self {
            pid,
            state: TaskState::Running,
            priority: 0,
            ticks: 0,
            context: TaskContext::default(),
            kernel_stack: [0u8; KERNEL_STACK_SIZE],
            kernel_sp: 0,
            page_table: kernel_table,
        }
    }
}

fn spsr_el0() -> u64 {
    const SPSR_EL0T: u64 = 0x00;
    const SPSR_D: u64 = 1 << 9;
    const SPSR_A: u64 = 1 << 8;
    const SPSR_I: u64 = 1 << 7;
    const SPSR_F: u64 = 1 << 6;
    SPSR_EL0T | SPSR_D | SPSR_A | SPSR_I | SPSR_F
}

fn spsr_el1() -> u64 {
    const SPSR_EL1H: u64 = 0x05;
    const SPSR_D: u64 = 1 << 9;
    const SPSR_A: u64 = 1 << 8;
    const SPSR_I: u64 = 1 << 7;
    const SPSR_F: u64 = 1 << 6;
    SPSR_EL1H | SPSR_D | SPSR_A | SPSR_I | SPSR_F
}
