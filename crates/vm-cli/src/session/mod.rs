mod account;
mod paths;
mod settings;
mod state;
mod util;

use std::collections::HashMap;

use anyhow::{Result, anyhow};
use vm_core::SessionRole;

pub use settings::UserSettings;
pub use state::SessionState;

use crate::session::account::{create_account_record, has_accounts, read_account};
use crate::session::settings::{read_user_settings, write_user_settings};
use crate::session::state::{clear_session_file, persist_session_file, restore_state};

pub struct CreateAccountResult {
    pub created_role: SessionRole,
    pub bootstrap_admin: bool,
}

pub fn restore(env: &HashMap<String, String>) -> Result<SessionState> {
    restore_state(env)
}

pub fn create_account(
    env: &HashMap<String, String>,
    actor: &SessionState,
    name: &str,
    password: &str,
    requested_role: SessionRole,
) -> Result<CreateAccountResult> {
    let has_any = has_accounts(env)?;
    let role = if !has_any {
        SessionRole::Admin
    } else if actor.role == SessionRole::Admin {
        requested_role
    } else {
        return Err(anyhow!("only admin can create additional accounts"));
    };

    create_account_record(env, name, password, role)?;
    write_user_settings(env, name, &UserSettings::default())?;
    Ok(CreateAccountResult {
        created_role: role,
        bootstrap_admin: !has_any,
    })
}

pub fn login(env: &HashMap<String, String>, name: &str, password: &str) -> Result<SessionState> {
    let account = read_account(env, name)?;
    if account.password_hash != util::hash_password(password) {
        return Err(anyhow!("invalid credentials"));
    }
    let settings = read_user_settings(env, name)?;
    let state = SessionState::for_user(name, account.role);
    persist_session_file(env, &state, settings.session_ttl_secs)?;
    Ok(state)
}

pub fn logout(
    env: &HashMap<String, String>,
    state: &SessionState,
    password: Option<&str>,
) -> Result<()> {
    if let Some(user) = state.user.as_deref() {
        let settings = read_user_settings(env, user)?;
        if settings.logout_requires_password {
            let given = password.ok_or_else(|| anyhow!("password required for logout"))?;
            let account = read_account(env, user)?;
            if account.password_hash != util::hash_password(given) {
                return Err(anyhow!("invalid logout password"));
            }
        }
    }
    clear_session_file(env)
}

pub fn refresh(env: &HashMap<String, String>, state: &SessionState) -> Result<()> {
    if let Some(user) = state.user.as_deref() {
        let settings = read_user_settings(env, user)?;
        persist_session_file(env, state, settings.session_ttl_secs)?;
    }
    Ok(())
}

pub fn read_settings(env: &HashMap<String, String>, user: &str) -> Result<UserSettings> {
    read_user_settings(env, user)
}

pub fn write_settings(env: &HashMap<String, String>, user: &str, settings: &UserSettings) -> Result<()> {
    write_user_settings(env, user, settings)
}
