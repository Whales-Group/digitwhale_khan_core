use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::session::paths::settings_file;

pub struct UserSettings {
    pub session_ttl_secs: u64,
    pub logout_requires_password: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            session_ttl_secs: 86_400,
            logout_requires_password: true,
        }
    }
}

pub fn read_user_settings(env: &HashMap<String, String>, user: &str) -> Result<UserSettings> {
    let path = settings_file(env, user);
    let Ok(raw) = fs::read_to_string(path) else {
        return Ok(UserSettings::default());
    };
    let map = parse_kv(&raw);
    Ok(UserSettings {
        session_ttl_secs: map.get("session_ttl_secs").and_then(|v| v.parse().ok()).unwrap_or(86_400),
        logout_requires_password: map.get("logout_requires_password").map(|v| v == "true").unwrap_or(true),
    })
}

pub fn write_user_settings(
    env: &HashMap<String, String>,
    user: &str,
    settings: &UserSettings,
) -> Result<()> {
    let path = settings_file(env, user);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let raw = format!(
        "session_ttl_secs={}\nlogout_requires_password={}\n",
        settings.session_ttl_secs, settings.logout_requires_password
    );
    fs::write(path, raw)?;
    Ok(())
}

fn parse_kv(raw: &str) -> HashMap<String, String> {
    raw.lines()
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}
