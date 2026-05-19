use crate::Memory;

use crate::layer::{Layer, LayerError, LayerEvent};

// Lightweight layer that can be expanded into richer auditing later.
pub struct LoggerLayer;

impl Layer for LoggerLayer {
    fn name(&self) -> &'static str {
        "logger"
    }

    fn on_event(&mut self, _event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        Ok(())
    }
}
