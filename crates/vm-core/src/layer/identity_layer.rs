use crate::Memory;

use super::{Layer, LayerError, LayerEvent};

pub struct PlatformIdentityLayer {
    name: &'static str,
}

impl PlatformIdentityLayer {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Layer for PlatformIdentityLayer {
    fn name(&self) -> &'static str {
        "platform-profile"
    }

    fn on_event(&mut self, _event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        let _ = self.name;
        Ok(())
    }
}
