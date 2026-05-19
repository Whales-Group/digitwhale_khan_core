use sha2::{Digest, Sha256};
use vm_core::SessionRole;

pub fn hash_password(password: &str) -> String {
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    format!("{:x}", h.finalize())
}

pub fn parse_role(raw: &str) -> SessionRole {
    match raw {
        "admin" => SessionRole::Admin,
        "user" => SessionRole::User,
        _ => SessionRole::Guest,
    }
}

pub fn role_name(role: SessionRole) -> &'static str {
    match role {
        SessionRole::Admin => "admin",
        SessionRole::User => "user",
        SessionRole::Guest => "guest",
    }
}
