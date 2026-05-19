use vm_core::{
    boot_vm, Instruction, PlatformProfile, Program, RoundRobinScheduler, SessionRole,
    StdoutLogger, VmTask, SYSCALL_CREATE_LAYER,
};

#[test]
fn scheduler_runs_all_tasks() {
    let mut scheduler = RoundRobinScheduler::default();
    scheduler.push(VmTask::new(
        "t1",
        Program::new(vec![Instruction::LoadImm(1), Instruction::Halt]),
    ));
    scheduler.push(VmTask::new(
        "t2",
        Program::new(vec![Instruction::LoadImm(2), Instruction::Halt]),
    ));

    let mut vm = boot_vm(256, PlatformProfile::Core, SessionRole::Admin);
    let mut logger = StdoutLogger::default();
    let completed = scheduler
        .run_all(&mut vm, &mut logger)
        .expect("scheduler should run");

    assert_eq!(completed, vec!["t1".to_string(), "t2".to_string()]);
}

#[test]
fn kernel_syscall_can_create_user_layer() {
    let mut vm = boot_vm(256, PlatformProfile::Core, SessionRole::Admin);
    vm.reset();
    vm.set_syscall_args(vec!["usr-extra".to_string(), "320".to_string()]);

    let mut logger = StdoutLogger::default();
    let program = Program::new(vec![Instruction::Syscall(SYSCALL_CREATE_LAYER), Instruction::Halt]);
    vm.run(&program, &mut logger).expect("vm run should succeed");

    let layer_names: Vec<String> = vm
        .layers_mut()
        .order()
        .into_iter()
        .map(|(name, _, _)| name.to_string())
        .collect();

    assert!(layer_names.contains(&"usr".to_string()));
}
