use super::{LayerError, LayerInit, LayerStage, LayeredOrchestrator};

impl LayeredOrchestrator {
    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> Result<(), LayerError> {
        let idx = self.find_index(name).ok_or(LayerError::NotFound)?;
        let slot = self.slots[idx].as_mut().expect("slot");
        if slot.locked {
            return Err(LayerError::Locked);
        }
        slot.enabled = enabled;
        self.sync_registry_at(idx);
        Ok(())
    }

    pub fn set_priority(&mut self, name: &str, priority: u16) -> Result<(), LayerError> {
        let idx = self.find_index(name).ok_or(LayerError::NotFound)?;
        let slot = self.slots[idx].as_mut().expect("slot");
        if slot.locked {
            return Err(LayerError::Locked);
        }
        slot.priority = priority;
        self.sync_registry_at(idx);
        self.sort_by_priority();
        Ok(())
    }

    pub fn remove_layer(&mut self, name: &str) -> Result<(), LayerError> {
        let idx = self.find_index(name).ok_or(LayerError::NotFound)?;
        if self.slots[idx].map(|l| l.locked).unwrap_or(false) {
            return Err(LayerError::Locked);
        }
        shift_left(&mut self.slots, idx, self.len);
        shift_left(&mut self.registry, idx, self.len);
        self.len -= 1;
        self.slots[self.len] = None;
        self.registry[self.len] = None;
        Ok(())
    }

    pub fn create_user_layer(
        &mut self,
        name: &'static str,
        stage: LayerStage,
        priority: u16,
        init: LayerInit,
    ) -> Result<(), LayerError> {
        self.add_user(super::LayerDesc {
            name,
            priority,
            locked: false,
            enabled: true,
            stage,
            on_init: init,
            on_before: None,
            on_after: None,
        })
    }
}

fn shift_left<T: Copy>(slots: &mut [Option<T>; 16], from: usize, len: usize) {
    let mut i = from;
    while i + 1 < len {
        slots[i] = slots[i + 1];
        i += 1;
    }
}
