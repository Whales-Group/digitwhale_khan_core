use std::fs;

use crate::Memory;

pub const SYSCALL_PRINT: u16 = 1;
pub const SYSCALL_ECHO: u16 = 10;
pub const SYSCALL_LS: u16 = 11;
pub const SYSCALL_CAT: u16 = 12;
pub const SYSCALL_PROC: u16 = 20;
pub const SYSCALL_DEVICE: u16 = 30;
pub const SYSCALL_CREATE_LAYER: u16 = 40;

// Handles built-in host abstractions for VM syscalls.
pub fn handle_syscall(id: u16, args: &[String], memory: &Memory, acc: i64) -> String {
    match id {
        SYSCALL_PRINT => format!("ACC={acc}"),
        SYSCALL_ECHO => args.join(" "),
        SYSCALL_LS => syscall_ls(args.first().map(String::as_str).unwrap_or(".")),
        SYSCALL_CAT => syscall_cat(args.first().map(String::as_str).unwrap_or("")),
        SYSCALL_PROC => format!("process stub: acc={acc} mem={}", memory.len()),
        SYSCALL_DEVICE => "device stub: virtual-device-0".to_string(),
        _ => format!("unknown syscall id={id}"),
    }
}

fn syscall_ls(path: &str) -> String {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        Err(e) => format!("error: {e}"),
    }
}

fn syscall_cat(path: &str) -> String {
    if path.is_empty() {
        return "error: missing file path".to_string();
    }
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => format!("error: {e}"),
    }
}
