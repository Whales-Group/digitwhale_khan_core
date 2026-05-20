use core::sync::atomic::{AtomicBool, Ordering};

use super::frame::{FRAME_SIZE, allocate_frame, free_frame, init_frame_allocator, used_frames};

static TEST_LOCK: AtomicBool = AtomicBool::new(false);

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        TEST_LOCK.store(false, Ordering::Release);
    }
}

fn lock_tests() -> Guard {
    while TEST_LOCK
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        core::hint::spin_loop();
    }
    // Reset frame allocator state after acquiring lock to ensure clean state
    init_frame_allocator();
    Guard
}

#[test]
fn allocates_in_deterministic_order() {
    let _guard = lock_tests();
    assert_eq!(allocate_frame(), Some(0));
    assert_eq!(allocate_frame(), Some(FRAME_SIZE));
    assert_eq!(allocate_frame(), Some(FRAME_SIZE * 2));
}

#[test]
fn reuses_freed_frame_deterministically() {
    let _guard = lock_tests();
    let a = allocate_frame().expect("first");
    let b = allocate_frame().expect("second");
    free_frame(a);
    assert_eq!(allocate_frame(), Some(a));
    assert_eq!(used_frames(), 2);
    assert_eq!(b, FRAME_SIZE);
}

#[test]
fn returns_none_when_exhausted() {
    let _guard = lock_tests();
    let mut count = 0;
    while let Some(addr) = allocate_frame() {
        assert_eq!(addr, count * FRAME_SIZE);
        count += 1;
    }
    assert!(count > 0);
}
