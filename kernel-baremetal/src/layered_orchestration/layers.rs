use crate::memory;

#[cfg(not(test))]
use crate::{drivers, interrupts, task};

use super::{LayerDesc, LayerError, LayerStage, LayeredOrchestrator};

pub fn default_orchestrator() -> Result<LayeredOrchestrator, LayerError> {
    let mut orch = LayeredOrchestrator::new();
    orch.add_system(base("memory", 10, LayerStage::Memory, init_memory))?;
    orch.add_system(base(
        "interrupts",
        20,
        LayerStage::Interrupts,
        init_interrupts,
    ))?;
    orch.add_system(base("drivers", 30, LayerStage::Drivers, init_drivers))?;
    orch.add_system(base("task", 40, LayerStage::Tasking, init_task))?;
    Ok(orch)
}

fn init_memory() -> Result<(), LayerError> {
    memory::init_memory();
    Ok(())
}
fn init_interrupts() -> Result<(), LayerError> {
    #[cfg(not(test))]
    {
        interrupts::init_exception_vectors();
        interrupts::init_gic();
    }
    #[cfg(test)]
    {
        crate::kernel_log::info("interrupts: skipped in test");
    }
    Ok(())
}
fn init_drivers() -> Result<(), LayerError> {
    #[cfg(not(test))]
    {
        drivers::init_early_drivers();
    }
    #[cfg(test)]
    {
        crate::kernel_log::info("drivers: skipped in test");
    }
    Ok(())
}
fn init_task() -> Result<(), LayerError> {
    #[cfg(not(test))]
    {
        task::init_scheduler();
        crate::init::spawn_init();
    }
    #[cfg(test)]
    {
        crate::kernel_log::info("task: skipped in test");
    }
    Ok(())
}

fn base(
    name: &'static str,
    priority: u16,
    stage: LayerStage,
    on_init: fn() -> Result<(), LayerError>,
) -> LayerDesc {
    LayerDesc {
        name,
        priority,
        locked: true,
        enabled: true,
        stage,
        on_init,
        on_before: None,
        on_after: None,
    }
}
