use std::collections::HashMap;
use std::fs;

use anyhow::{Result, anyhow};
use vm_core::SessionRole;

use crate::session::paths::accounts_file;
use crate::session::util::{hash_password, parse_role, role_name};

pub struct AccountRecord {
    pub password_hash: String,
    pub role: SessionRole,
}

pub fn has_accounts(env: &HashMap<String, String>) -> Result<bool> {
    Ok(read_lines(&accounts_file(env))?
        .iter()
        .any(|line| !line.trim().is_empty()))
}

pub fn create_account_record(
    env: &HashMap<String, String>,
    name: &str,
    password: &str,
    role: SessionRole,
) -> Result<()> {
    if name.is_empty() || password.is_empty() {
        return Err(anyhow!("name/password cannot be empty"));
    }
    let path = accounts_file(env);
    let mut lines = read_lines(&path)?;
    if lines.iter().any(|l| l.split(':').next() == Some(name)) {
        return Err(anyhow!("account already exists"));
    }
    lines.push(format!("{name}:{}:{}", hash_password(password), role_name(role)));
    fs::write(path, lines.join("\n") + "\n")?;
    Ok(())
}

pub fn read_account(env: &HashMap<String, String>, user: &str) -> Result<AccountRecord> {
    for line in read_lines(&accounts_file(env))? {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 3 && parts[0] == user {
            return Ok(AccountRecord {
                password_hash: parts[1].to_string(),
                role: parse_role(parts[2]),
            });
        }
    }
    Err(anyhow!("account not found"))
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    let Ok(raw) = fs::read_to_string(path) else {
        return Ok(vec![]);
    };
    Ok(raw.lines().map(str::to_string).collect())
}
