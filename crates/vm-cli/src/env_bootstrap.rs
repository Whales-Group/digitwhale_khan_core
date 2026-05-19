use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

use anyhow::Result;

pub fn ensure_env_and_usr() -> Result<HashMap<String, String>> {
    if !Path::new(".env").exists() {
        fs::write(".env", default_env())?;
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
    let history = env_map
        .get("JEVAN_HISTORY_FILE")
        .map(String::as_str)
        .unwrap_or(".usr/history/commands.history");
    let accounts = env_map
        .get("JEVAN_ACCOUNTS_FILE")
        .map(String::as_str)
        .unwrap_or(".usr/config/accounts.db");
    let session = env_map
        .get("JEVAN_SESSION_FILE")
        .map(String::as_str)
        .unwrap_or(".usr/config/session.db");
    let user_settings = env_map
        .get("JEVAN_USER_SETTINGS_DIR")
        .map(String::as_str)
        .unwrap_or(".usr/config/users");
    let settings_layer = env_map
        .get("JEVAN_SETTINGS_LAYER_FILE")
        .map(String::as_str)
        .unwrap_or(".usr/config/settings.layer");

    fs::create_dir_all(root)?;
    fs::create_dir_all(config)?;
    if let Some(parent) = Path::new(history).parent() {
        fs::create_dir_all(parent)?;
    }
    if !Path::new(history).exists() {
        fs::write(history, "")?;
    }
    if !Path::new(accounts).exists() {
        fs::write(accounts, "")?;
    }
    if !Path::new(session).exists() {
        fs::write(session, "")?;
    }
    fs::create_dir_all(user_settings)?;
    if !Path::new(settings_layer).exists() {
        fs::write(settings_layer, "session_store=enabled\naccount_store=enabled\n")?;
    }
    if !Path::new(layers).exists() {
        fs::write(
            layers,
            "layer.platform-profile=on\nlayer.env=on\nlayer.settings=on\nlayer.logger=on\nlayer.fs=on\nlayer.usr=on\nlayer.security=on\nlayer.capability=on\n",
        )?;
    }

    Ok(env_map)
}

fn default_env() -> String {
    let instance = instance_id();
    let root = format!(".usr/instances/{instance}");
    format!(
        "JEVAN_INSTANCE_ID={instance}\nJEVAN_ROOT={root}\nJEVAN_CONFIG_DIR={root}/config\nJEVAN_LAYER_CONFIG={root}/config/layers.conf\nJEVAN_HISTORY_FILE={root}/history/commands.history\nJEVAN_ACCOUNTS_FILE={root}/config/accounts.db\nJEVAN_SESSION_FILE={root}/config/session.db\nJEVAN_USER_SETTINGS_DIR={root}/config/users\nJEVAN_SETTINGS_LAYER_FILE={root}/config/settings.layer\n"
    )
}

fn instance_id() -> String {
    let cwd = std::env::current_dir().unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    cwd.to_string_lossy().hash(&mut hasher);
    format!("{:x}", hasher.finish())
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
