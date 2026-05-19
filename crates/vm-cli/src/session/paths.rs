use std::collections::HashMap;

pub fn accounts_file(env: &HashMap<String, String>) -> String {
    env.get("JEVAN_ACCOUNTS_FILE")
        .cloned()
        .unwrap_or_else(|| ".usr/config/accounts.db".to_string())
}

pub fn session_file(env: &HashMap<String, String>) -> String {
    env.get("JEVAN_SESSION_FILE")
        .cloned()
        .unwrap_or_else(|| ".usr/config/session.db".to_string())
}

pub fn settings_file(env: &HashMap<String, String>, user: &str) -> String {
    env.get("JEVAN_USER_SETTINGS_DIR")
        .cloned()
        .map(|d| format!("{d}/{user}.settings"))
        .unwrap_or_else(|| format!(".usr/config/users/{user}.settings"))
}
