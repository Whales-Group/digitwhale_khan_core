use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 256 * 1024;

struct BumpAllocator {
    heap_start: AtomicUsize,
    heap_end: AtomicUsize,
    next: AtomicUsize,
}

static ALLOCATOR: BumpAllocator = BumpAllocator {
    heap_start: AtomicUsize::new(0),
    heap_end: AtomicUsize::new(0),
    next: AtomicUsize::new(0),
};

pub fn init_heap() {
    if ALLOCATOR.heap_start.load(Ordering::SeqCst) != 0 {
        return;
    }

    let pages = HEAP_SIZE / crate::memory::frame::FRAME_SIZE;
    let first = match crate::memory::frame::allocate_frame() {
        Some(a) => a,
        None => {
            crate::kernel_log::info("heap: no frames available");
            return;
        }
    };

    let mut end = first + crate::memory::frame::FRAME_SIZE;
    let mut remaining = pages - 1;
    while remaining > 0 {
        let expected = end;
        match crate::memory::frame::allocate_frame() {
            Some(addr) if addr == expected => {
                end = addr + crate::memory::frame::FRAME_SIZE;
                remaining -= 1;
            }
            Some(addr) => {
                crate::memory::frame::free_frame(addr);
                break;
            }
            None => break,
        }
    }

    ALLOCATOR.heap_start.store(first, Ordering::SeqCst);
    ALLOCATOR.heap_end.store(end, Ordering::SeqCst);
    ALLOCATOR.next.store(first, Ordering::SeqCst);
}

pub fn heap_alloc(size: usize, align: usize) -> Option<*mut u8> {
    let start = ALLOCATOR.heap_start.load(Ordering::SeqCst);
    if start == 0 {
        return None;
    }
    let heap_end = ALLOCATOR.heap_end.load(Ordering::SeqCst);
    let align = if align < 16 { 16 } else { align };

    loop {
        let current = ALLOCATOR.next.load(Ordering::SeqCst);
        let aligned = (current + align - 1) & !(align - 1);
        let new_next = aligned + size;
        if new_next > heap_end {
            return None;
        }
        if ALLOCATOR
            .next
            .compare_exchange_weak(current, new_next, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return Some(aligned as *mut u8);
        }
    }
}
