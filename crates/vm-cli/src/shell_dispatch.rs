use std::collections::HashMap;

use anyhow::Result;
use vm_core::{Logger, PlatformProfile, SessionRole, Vm, boot_vm};

use crate::cli::Args;
use crate::commands;
use crate::password_prompt::prompt_password;
use crate::session::{self, SessionState, UserSettings};

pub fn dispatch(
    vm: &mut Vm,
    args: &Args,
    state: &mut SessionState,
    logger: &mut dyn Logger,
    input: &str,
    env: &HashMap<String, String>,
    apply_layers: impl Fn(&mut Vm),
) -> Result<bool> {
    let mut parts = input.split_whitespace();
    let cmd = parts.next().unwrap_or_default();
    let argsv: Vec<&str> = parts.collect();

    match cmd {
        "help" => commands::print_help(),
        "env" | "env-show" => commands::print_env(env),
        "account" => account_cmd(state, env, &argsv),
        "login" => login_cmd(vm, args, state, env, &argsv, apply_layers),
        "logout" => logout_cmd(vm, args, state, env, &argsv, apply_layers),
        "settings" => settings_cmd(state, env, &argsv),
        "exit" | "quit" => return Ok(false),
        _ if matches!(state.role, SessionRole::Guest) => println!("guest locked: login first"),
        "demo" | "run-demo" => run_demo(vm, logger),
        "run" => run(vm, logger, &argsv),
        "status" => commands::print_status(vm),
        "layers" => commands::print_layers(vm),
        "layer" | "layer-set" => layer_cmd(vm, &argsv),
        "mem" => mem(vm, &argsv),
        "ls" => syscall_ls(vm, logger, &argsv),
        "cat" => syscall_cat(vm, logger, &argsv),
        "echo" => syscall_echo(vm, logger, &argsv),
        "mklayer" | "layer-create" => layer_create(vm, logger, &argsv),
        _ => println!("unknown command: {cmd}"),
    }
    let _ = session::refresh(env, state);
    Ok(true)
}

fn account_cmd(state: &SessionState, env: &HashMap<String, String>, args: &[&str]) {
    if args.len() >= 3 && args[0] == "create" {
        let role = match args[2] { "admin" => SessionRole::Admin, "user" => SessionRole::User, _ => SessionRole::Guest };
        let Ok(password) = prompt_password("password: ") else { return println!("error: password input failed"); };
        match session::create_account(env, state, args[1], &password, role) {
            Ok(result) if result.bootstrap_admin => println!("bootstrap account created as admin"),
            Ok(result) => println!("account created with role={:?}", result.created_role),
            Err(e) => println!("error: {e}"),
        }
    } else {
        println!("usage: account create <name> <admin|user|guest>");
    }
}

fn login_cmd(vm: &mut Vm, args: &Args, state: &mut SessionState, env: &HashMap<String, String>, argv: &[&str], apply_layers: impl Fn(&mut Vm)) {
    if argv.is_empty() { return println!("usage: login <name>"); }
    let Ok(password) = prompt_password("password: ") else { return println!("error: password input failed"); };
    match session::login(env, argv[0], &password) {
        Ok(next) => { *state = next; *vm = boot_vm(args.memory_size, PlatformProfile::Core, state.role); apply_layers(vm); println!("session unlocked as {}", argv[0]); }
        Err(e) => println!("error: {e}"),
    }
}

fn logout_cmd(vm: &mut Vm, args: &Args, state: &mut SessionState, env: &HashMap<String, String>, argv: &[&str], apply_layers: impl Fn(&mut Vm)) {
    let password = if argv.is_empty() {
        prompt_password("password (if required): ").ok()
    } else {
        Some(argv[0].to_string())
    };
    match session::logout(env, state, password.as_deref()) {
        Ok(()) => { *state = SessionState::guest(); *vm = boot_vm(args.memory_size, PlatformProfile::Core, state.role); apply_layers(vm); println!("logged out"); }
        Err(e) => println!("error: {e}"),
    }
}

fn settings_cmd(state: &SessionState, env: &HashMap<String, String>, args: &[&str]) {
    let Some(user) = state.user.as_deref() else { return println!("error: login required"); };
    if args.is_empty() || args[0] == "show" {
        return match session::read_settings(env, user) {
            Ok(s) => println!("session_ttl_secs={} logout_requires_password={}", s.session_ttl_secs, s.logout_requires_password),
            Err(e) => println!("error: {e}"),
        };
    }
    if args.len() < 3 || args[0] != "set" { return println!("usage: settings show | settings set <session-ttl|logout-password> <value>"); }
    let mut s = session::read_settings(env, user).unwrap_or(UserSettings::default());
    match args[1] { "session-ttl" => s.session_ttl_secs = args[2].parse().unwrap_or(s.session_ttl_secs), "logout-password" => s.logout_requires_password = args[2].eq_ignore_ascii_case("on"), _ => return println!("unknown setting: {}", args[1]), }
    if let Err(e) = session::write_settings(env, user, &s) { println!("error: {e}"); } else { println!("settings updated"); }
}

fn run_demo(vm: &mut Vm, logger: &mut dyn Logger) { if let Err(e) = commands::run_demo(vm, logger) { println!("error: {e}"); } }
fn run(vm: &mut Vm, logger: &mut dyn Logger, argv: &[&str]) { if let Some(path) = argv.first() { if let Err(e) = commands::run_bytecode(vm, path, logger) { println!("error: {e}"); } } else { println!("usage: run <bytecode_file.vmb>"); } }
fn mem(vm: &Vm, argv: &[&str]) { if let Some(raw) = argv.first() { match raw.parse::<usize>() { Ok(a) => commands::print_mem(vm, a), Err(_) => println!("usage: mem <addr>") } } else { println!("usage: mem <addr>"); } }
fn layer_cmd(vm: &mut Vm, argv: &[&str]) { if argv.len() >= 3 && argv[0] == "set" { commands::layer_set(vm, argv[1], argv[2].eq_ignore_ascii_case("on")); } else if argv.len() >= 2 { commands::layer_set(vm, argv[0], argv[1].eq_ignore_ascii_case("on")); } else { println!("usage: layer set <name> <on|off>"); } }
fn syscall_ls(vm: &mut Vm, logger: &mut dyn Logger, argv: &[&str]) { if let Err(e) = commands::syscall_ls(vm, argv.first().copied(), logger) { println!("error: {e}"); } }
fn syscall_cat(vm: &mut Vm, logger: &mut dyn Logger, argv: &[&str]) { if let Some(path) = argv.first() { if let Err(e) = commands::syscall_cat(vm, path, logger) { println!("error: {e}"); } } else { println!("usage: cat <file>"); } }
fn syscall_echo(vm: &mut Vm, logger: &mut dyn Logger, argv: &[&str]) { if let Err(e) = commands::syscall_echo(vm, argv, logger) { println!("error: {e}"); } }
fn layer_create(vm: &mut Vm, logger: &mut dyn Logger, argv: &[&str]) { if argv.len() >= 2 { if let Ok(p) = argv[1].parse::<i32>() { if let Err(e) = commands::syscall_create_layer(vm, argv[0], p, logger) { println!("error: {e}"); } } else { println!("usage: mklayer <name> <priority>"); } } else { println!("usage: mklayer <name> <priority>"); } }
