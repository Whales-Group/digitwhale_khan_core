use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::state::{Task, TaskState};
use super::context::TaskContext;

const MAX_TASKS: usize = 64;
const TIMESLICE: u32 = 10;

#[unsafe(no_mangle)]
extern "C" fn idle_loop() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

static mut TASKS: [Option<Task>; MAX_TASKS] = [const { None }; MAX_TASKS];
static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);
static CURRENT_IDX: AtomicUsize = AtomicUsize::new(0);
static SCHEDULER_READY: AtomicBool = AtomicBool::new(false);

pub fn init_scheduler() {
    let mut idle = Task::idle();
    idle.context.x[30] = idle_loop as *const () as usize as u64;
    idle.context.sp = idle.kernel_stack.as_ptr() as u64 + crate::task::state::KERNEL_STACK_SIZE as u64;
    unsafe {
        TASKS[0] = Some(idle);
    }
    TASK_COUNT.store(1, Ordering::SeqCst);
    CURRENT_IDX.store(0, Ordering::SeqCst);
    SCHEDULER_READY.store(true, Ordering::SeqCst);
}

pub fn spawn(entry: u64, priority: u8, el0: bool) -> Option<u32> {
    if !SCHEDULER_READY.load(Ordering::SeqCst) {
        return None;
    }

    let count = TASK_COUNT.load(Ordering::SeqCst);
    if count >= MAX_TASKS {
        return None;
    }

    let task = Task::new(entry, priority, el0);
    let pid = task.pid;

    let idx = count;
    unsafe {
        TASKS[idx] = Some(task);
    }
    TASK_COUNT.store(count + 1, Ordering::SeqCst);

    Some(pid)
}

pub fn schedule() {
    if !SCHEDULER_READY.load(Ordering::SeqCst) {
        return;
    }

    let count = TASK_COUNT.load(Ordering::SeqCst);
    if count <= 1 {
        return;
    }

    let current = CURRENT_IDX.load(Ordering::SeqCst);

    unsafe {
        if let Some(ref mut cur) = TASKS[current] {
            if cur.state == TaskState::Running {
                cur.state = TaskState::Ready;
            }
        }
    }

    let mut next = (current + 1) % count;
    let mut tries = 0;
    loop {
        unsafe {
            if let Some(ref task) = TASKS[next] {
                if task.state == TaskState::Ready {
                    break;
                }
            }
        }
        next = (next + 1) % count;
        tries += 1;
        if tries >= count {
            next = 0;
            break;
        }
    }

    CURRENT_IDX.store(next, Ordering::SeqCst);

    unsafe {
        if let Some(ref mut task) = TASKS[next] {
            task.state = TaskState::Running;
            task.ticks = 0;
        }

        let old_task = TASKS[current].as_mut().unwrap();
        let new_task = TASKS[next].as_ref().unwrap();

        if old_task.page_table != new_task.page_table {
            crate::memory::paging::switch_page_table(new_task.page_table);
        }

        crate::drivers::serial::write_str("[sched] switching to pid ");
        crate::drivers::serial::write_u64_dec(new_task.pid as u64);
        crate::drivers::serial::write_str("\n");

        let old_ctx = &mut old_task.context;
        let new_ctx = &new_task.context;

        crate::task::context::switch_context(old_ctx as *mut TaskContext, new_ctx);
    }
}

pub fn tick() {
    if !SCHEDULER_READY.load(Ordering::SeqCst) {
        return;
    }

    let current = CURRENT_IDX.load(Ordering::SeqCst);
    unsafe {
        if let Some(ref mut task) = TASKS[current] {
            task.ticks += 1;
            if task.ticks >= TIMESLICE {
                task.ticks = 0;
            }
        }
    }
}

pub fn exit_current() -> ! {
    let current = CURRENT_IDX.load(Ordering::SeqCst);
    unsafe {
        if let Some(ref mut task) = TASKS[current] {
            task.state = TaskState::Exited;
        }
    }
    schedule();
    loop {
        core::hint::spin_loop();
    }
}

pub fn block_current() {
    let current = CURRENT_IDX.load(Ordering::SeqCst);
    unsafe {
        if let Some(ref mut task) = TASKS[current] {
            task.state = TaskState::Blocked;
        }
    }
    schedule();
}

pub fn wake_by_pid(pid: u32) {
    let count = TASK_COUNT.load(Ordering::SeqCst);
    let mut i = 0;
    while i < count {
        unsafe {
            if let Some(ref task) = TASKS[i] {
                if task.pid == pid && task.state == TaskState::Blocked {
                    TASKS[i].as_mut().unwrap().state = TaskState::Ready;
                }
            }
        }
        i += 1;
    }
}

pub fn current_pid() -> u32 {
    let current = CURRENT_IDX.load(Ordering::SeqCst);
    unsafe {
        TASKS[current].as_ref().map(|t| t.pid).unwrap_or(0)
    }
}
