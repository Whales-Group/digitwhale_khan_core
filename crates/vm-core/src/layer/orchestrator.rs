use crate::Memory;

use super::{Layer, LayerError, LayerEvent};

struct LayerSlot {
    name: &'static str,
    enabled: bool,
    priority: i32,
    locked: bool,
    layer: Box<dyn Layer>,
}

#[derive(Default)]
pub struct LayerOrchestrator {
    layers: Vec<LayerSlot>,
}

impl LayerOrchestrator {
    pub fn add_system<L: Layer + 'static>(&mut self, layer: L, priority: i32) {
        self.add_inner(layer, priority, true);
    }

    pub fn add_user<L: Layer + 'static>(&mut self, layer: L, priority: i32) {
        self.add_inner(layer, priority, false);
    }

    pub fn add<L: Layer + 'static>(&mut self, layer: L, priority: i32) {
        self.add_user(layer, priority);
    }

    fn add_inner<L: Layer + 'static>(&mut self, layer: L, priority: i32, locked: bool) {
        let slot = LayerSlot {
            name: layer.name(),
            enabled: true,
            priority,
            locked,
            layer: Box::new(layer),
        };
        self.layers.push(slot);
        self.sort_layers();
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        for slot in &mut self.layers {
            if slot.name == name {
                slot.enabled = enabled;
            }
        }
    }

    pub fn set_priority(&mut self, name: &str, priority: i32) -> bool {
        for slot in &mut self.layers {
            if slot.name == name {
                if slot.locked {
                    return false;
                }
                slot.priority = priority;
                self.sort_layers();
                return true;
            }
        }
        false
    }

    pub fn order(&self) -> Vec<(&'static str, i32, bool)> {
        self.layers
            .iter()
            .map(|slot| (slot.name, slot.priority, slot.enabled))
            .collect()
    }

    pub fn notify(&mut self, event: &LayerEvent, memory: &Memory) -> Result<(), LayerError> {
        for slot in &mut self.layers {
            if slot.enabled {
                slot.layer.on_event(event, memory)?;
            }
        }
        Ok(())
    }

    fn sort_layers(&mut self) {
        self.layers
            .sort_by(|a, b| a.priority.cmp(&b.priority).then_with(|| a.name.cmp(b.name)));
    }
}
