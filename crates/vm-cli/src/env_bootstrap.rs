use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;

const DEFAULT_ENV: &str = "JEVAN_ROOT=.usr\nJEVAN_CONFIG_DIR=.usr/config\nJEVAN_LAYER_CONFIG=.usr/config/layers.conf\n";

pub fn ensure_env_and_usr() -> Result<HashMap<String, String>> {
    if !Path::new(".env").exists() {
        fs::write(".env", DEFAULT_ENV)?;
    }

    let env_map = load_env_map()?;
    let root = env_map.get("JEVAN_ROOT").map(String::as_str).unwrap_or(".usr");
    let config = env_map
        .get("JEVAN_CONFIG_DIR")
        .map(String::as_str)
        .unwrap_or(".usr/config");
    let layers = env_map
        .get("JEVAN_LAYER_CONFIG")
        .map(String::as_str)
        .unwrap_or(".usr/config/layers.conf");

    fs::create_dir_all(root)?;
    fs::create_dir_all(config)?;
    if !Path::new(layers).exists() {
        fs::write(
            layers,
            "layer.platform-profile=on\nlayer.usr=on\nlayer.logger=on\nlayer.security=on\nlayer.capability=on\n",
        )?;
    }

    Ok(env_map)
}

fn load_env_map() -> Result<HashMap<String, String>> {
    let raw = fs::read_to_string(".env")?;
    let mut map = HashMap::new();

    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    Ok(map)
}
