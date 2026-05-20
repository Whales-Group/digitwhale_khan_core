use super::{LayerError, LayerStage, LayeredOrchestrator};

impl LayeredOrchestrator {
    pub fn run_boot_sequence(&self) -> Result<(), LayerError> {
        let stages = [
            LayerStage::Boot,
            LayerStage::Memory,
            LayerStage::Interrupts,
            LayerStage::Drivers,
            LayerStage::Tasking,
        ];
        let mut i = 0;
        while i < stages.len() {
            run_stage(self, stages[i])?;
            i += 1;
        }
        Ok(())
    }
}

fn run_stage(orchestrator: &LayeredOrchestrator, stage: LayerStage) -> Result<(), LayerError> {
    call_hooks(orchestrator, stage, true)?;
    let mut i = 0;
    while i < orchestrator.len {
        if let Some(layer) = orchestrator.slots[i] {
            if layer.enabled && layer.stage == stage {
                (layer.on_init)().map_err(|_| LayerError::InitFailed(layer.name))?;
            }
        }
        i += 1;
    }
    call_hooks(orchestrator, stage, false)
}

fn call_hooks(
    orchestrator: &LayeredOrchestrator,
    stage: LayerStage,
    before: bool,
) -> Result<(), LayerError> {
    let mut i = 0;
    while i < orchestrator.len {
        if let Some(layer) = orchestrator.slots[i] {
            if layer.enabled {
                let hook = if before {
                    layer.on_before
                } else {
                    layer.on_after
                };
                if let Some(h) = hook {
                    h(stage).map_err(|_| LayerError::HookFailed(layer.name))?;
                }
            }
        }
        i += 1;
    }
    Ok(())
}
