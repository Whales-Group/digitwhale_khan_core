use super::{LayerDesc, LayerError, LayerKind, LayerStage, LayeredOrchestrator};
use core::sync::atomic::{AtomicUsize, Ordering};

fn ok() -> Result<(), LayerError> {
    Ok(())
}
static BEFORE_HITS: AtomicUsize = AtomicUsize::new(0);
static AFTER_HITS: AtomicUsize = AtomicUsize::new(0);

#[test]
fn locked_layers_cannot_be_mutated() {
    let mut o = LayeredOrchestrator::new();
    o.add_system(base("sys", 10, LayerStage::Boot))
        .expect("add");
    assert_eq!(o.set_enabled("sys", false), Err(LayerError::Locked));
    assert_eq!(o.set_priority("sys", 1), Err(LayerError::Locked));
    assert_eq!(o.remove_layer("sys"), Err(LayerError::Locked));
}

#[test]
fn user_layers_can_change_state_and_priority() {
    let mut o = LayeredOrchestrator::new();
    o.add_user(base("usr", 20, LayerStage::Boot)).expect("add");
    o.set_enabled("usr", false).expect("enabled");
    o.set_priority("usr", 5).expect("prio");
    let first = o.order()[0].expect("order slot");
    assert_eq!(first.0, "usr");
    assert!(!first.2);
}

#[test]
fn duplicate_registration_is_rejected() {
    let mut o = LayeredOrchestrator::new();
    o.add_user(base("dup", 10, LayerStage::Boot))
        .expect("first");
    let err = o
        .add_user(base("dup", 20, LayerStage::Drivers))
        .expect_err("dup");
    assert_eq!(err, LayerError::Duplicate);
}

#[test]
fn runtime_registration_table_tracks_kind_and_flags() {
    let mut o = LayeredOrchestrator::new();
    o.add_system(base("sys", 10, LayerStage::Boot))
        .expect("sys");
    o.create_user_layer("usr", LayerStage::Tasking, 50, ok)
        .expect("usr");

    let table = o.registration_table();
    let a = table[0].expect("row0");
    let b = table[1].expect("row1");
    assert_eq!(a.kind, LayerKind::System);
    assert!(a.immutable);
    assert_eq!(b.kind, LayerKind::User);
    assert!(!b.immutable);
}

#[test]
fn before_after_hooks_run_during_boot_sequence() {
    BEFORE_HITS.store(0, Ordering::SeqCst);
    AFTER_HITS.store(0, Ordering::SeqCst);
    let mut o = LayeredOrchestrator::new();
    o.add_user(hooked("h", 1, LayerStage::Boot)).expect("add");
    o.run_boot_sequence().expect("boot");
    assert!(BEFORE_HITS.load(Ordering::SeqCst) >= 1);
    assert!(AFTER_HITS.load(Ordering::SeqCst) >= 1);
}

fn base(name: &'static str, priority: u16, stage: LayerStage) -> LayerDesc {
    LayerDesc {
        name,
        priority,
        locked: false,
        enabled: true,
        stage,
        on_init: ok,
        on_before: None,
        on_after: None,
    }
}

fn hooked(name: &'static str, priority: u16, stage: LayerStage) -> LayerDesc {
    LayerDesc {
        name,
        priority,
        locked: false,
        enabled: true,
        stage,
        on_init: ok,
        on_before: Some(|_| {
            BEFORE_HITS.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }),
        on_after: Some(|_| {
            AFTER_HITS.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }),
    }
}
