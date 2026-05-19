use std::collections::VecDeque;

use crate::{Logger, Vm, VmError, VmTask};

#[derive(Default)]
pub struct RoundRobinScheduler {
    queue: VecDeque<VmTask>,
}

impl RoundRobinScheduler {
    pub fn push(&mut self, task: VmTask) {
        self.queue.push_back(task);
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn run_all(&mut self, vm: &mut Vm, logger: &mut dyn Logger) -> Result<Vec<String>, VmError> {
        let mut completed = vec![];

        while let Some(task) = self.queue.pop_front() {
            vm.reset();
            vm.set_syscall_args(task.syscall_args);
            vm.run(&task.program, logger)?;
            completed.push(task.name);
        }

        Ok(completed)
    }
}
