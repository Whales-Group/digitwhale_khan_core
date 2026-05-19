use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use vm_core::SessionRole;

use crate::session::paths::session_file;
use crate::session::util::{parse_role, role_name};

pub struct SessionState {
    pub user: Option<String>,
    pub role: SessionRole,
}

impl SessionState {
    pub fn guest() -> Self {
        Self { user: None, role: SessionRole::Guest }
    }

    pub fn for_user(user: &str, role: SessionRole) -> Self {
        Self { user: Some(user.to_string()), role }
    }
}

pub fn restore_state(env: &HashMap<String, String>) -> Result<SessionState> {
    let path = session_file(env);
    let Ok(raw) = fs::read_to_string(path) else {
        return Ok(SessionState::guest());
    };
    let map = parse_kv(&raw);
    let exp = map.get("expires_at").and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
    if now() >= exp {
        clear_session_file(env)?;
        return Ok(SessionState::guest());
    }
    let user = map.get("user").cloned().filter(|u| !u.is_empty());
    let role = parse_role(map.get("role").map(String::as_str).unwrap_or("guest"));
    Ok(SessionState { user, role })
}

pub fn persist_session_file(env: &HashMap<String, String>, state: &SessionState, ttl: u64) -> Result<()> {
    let path = session_file(env);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let user = state.user.clone().unwrap_or_default();
    let raw = format!("user={user}\nrole={}\nexpires_at={}\n", role_name(state.role), now() + ttl);
    fs::write(path, raw)?;
    Ok(())
}

pub fn clear_session_file(env: &HashMap<String, String>) -> Result<()> {
    let path = session_file(env);
    if Path::new(&path).exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn parse_kv(raw: &str) -> HashMap<String, String> {
    raw.lines()
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}
