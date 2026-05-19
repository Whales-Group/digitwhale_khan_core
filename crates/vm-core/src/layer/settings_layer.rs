use std::collections::HashMap;
use std::fs;

use crate::Memory;

use super::{Layer, LayerError, LayerEvent};

pub struct SettingsLayer {
    values: HashMap<String, String>,
}

impl SettingsLayer {
    pub fn from_dotenv(dotenv: &str) -> Self {
        let path = fs::read_to_string(dotenv)
            .ok()
            .and_then(|raw| parse_env_value(&raw, "JEVAN_SETTINGS_LAYER_FILE"))
            .unwrap_or_else(|| ".usr/config/settings.layer".to_string());
        let values = fs::read_to_string(path).map(|raw| parse_kv(&raw)).unwrap_or_default();
        Self { values }
    }
}

impl Layer for SettingsLayer {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn on_event(&mut self, _event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        let _loaded = self.values.len();
        Ok(())
    }
}

fn parse_env_value(raw: &str, key: &str) -> Option<String> {
    raw.lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix(&format!("{key}=")).map(str::to_string))
}

fn parse_kv(raw: &str) -> HashMap<String, String> {
    raw.lines()
        .filter_map(|line| line.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}
