use std::fs;

use crate::Memory;

use super::{Layer, LayerError, LayerEvent};

pub struct EnvLayer {
    env: Vec<(String, String)>,
}

impl EnvLayer {
    pub fn from_dotenv(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap_or_default();
        let env = contents
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .filter_map(|l| {
                let (key, val) = l.split_once('=')?;
                Some((key.trim().to_string(), val.trim().to_string()))
            })
            .collect();

        Self { env }
    }
}

impl Layer for EnvLayer {
    fn name(&self) -> &'static str {
        "env"
    }

    fn on_event(&mut self, _event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        let _loaded_vars = self.env.len();
        Ok(())
    }
}
