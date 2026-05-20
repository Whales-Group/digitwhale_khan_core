use super::{LayerDesc, LayerError, LayerKind, LayerRegistration};

pub struct LayeredOrchestrator {
    pub(crate) slots: [Option<LayerDesc>; 16],
    pub(crate) registry: [Option<LayerRegistration>; 16],
    pub(crate) len: usize,
}

impl LayeredOrchestrator {
    pub const fn new() -> Self {
        Self {
            slots: [None; 16],
            registry: [None; 16],
            len: 0,
        }
    }

    pub fn add_system(&mut self, mut layer: LayerDesc) -> Result<(), LayerError> {
        layer.locked = true;
        self.add(layer, LayerKind::System)
    }

    pub fn add_user(&mut self, mut layer: LayerDesc) -> Result<(), LayerError> {
        layer.locked = false;
        self.add(layer, LayerKind::User)
    }

    pub fn registration_table(&self) -> [Option<LayerRegistration>; 16] {
        self.registry
    }

    pub fn order(&self) -> [Option<(&'static str, u16, bool, bool)>; 16] {
        let mut out = [None; 16];
        let mut i = 0;
        while i < self.len {
            if let Some(l) = self.slots[i] {
                out[i] = Some((l.name, l.priority, l.enabled, l.locked));
            }
            i += 1;
        }
        out
    }

    pub(crate) fn add(&mut self, layer: LayerDesc, kind: LayerKind) -> Result<(), LayerError> {
        if self.len >= self.slots.len() {
            return Err(LayerError::Capacity);
        }
        if self.find_index(layer.name).is_some() {
            return Err(LayerError::Duplicate);
        }

        self.slots[self.len] = Some(layer);
        self.registry[self.len] = Some(LayerRegistration {
            name: layer.name,
            kind,
            immutable: layer.locked,
            enabled: layer.enabled,
            priority: layer.priority,
            stage: layer.stage,
        });
        self.len += 1;
        self.sort_by_priority();
        Ok(())
    }

    pub(crate) fn find_index(&self, name: &str) -> Option<usize> {
        let mut i = 0;
        while i < self.len {
            if self.slots[i].map(|l| l.name == name).unwrap_or(false) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub(crate) fn sync_registry_at(&mut self, idx: usize) {
        if let (Some(slot), Some(reg)) = (self.slots[idx], self.registry[idx].as_mut()) {
            reg.enabled = slot.enabled;
            reg.priority = slot.priority;
            reg.stage = slot.stage;
            reg.immutable = slot.locked;
        }
    }

    pub(crate) fn sort_by_priority(&mut self) {
        let mut i = 0;
        while i < self.len {
            let mut j = i + 1;
            while j < self.len {
                let a = self.slots[i].unwrap();
                let b = self.slots[j].unwrap();
                if b.priority < a.priority {
                    self.slots.swap(i, j);
                    self.registry.swap(i, j);
                }
                j += 1;
            }
            i += 1;
        }
    }
}
