use std::env;
use std::fs;
use crate::Memory;
use super::{Layer, LayerError, LayerEvent};

pub struct EnvLayer {
    env: Vec<(String, String)>,
}

impl EnvLayer {
    pub fn new() -> Self {
        Self {
            env: env::vars().collect(),
        }
    }

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

    fn on_event(&mut self, event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        match event {
            _ => Ok(()),
        }
    }
}