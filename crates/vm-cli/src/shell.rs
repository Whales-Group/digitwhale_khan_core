use std::collections::HashMap;
use std::io::{self, Write};

use anyhow::Result;
use vm_core::{boot_vm, Logger, PlatformProfile, SessionRole, Vm};

use crate::cli::Args;
use crate::commands;
use crate::config::load_layer_config;
use crate::env_bootstrap::ensure_env_and_usr;

pub fn run_shell(args: Args, logger: &mut dyn Logger) -> Result<()> {
    let env = ensure_env_and_usr()?;
    let layer_cfg = env
        .get("JEVAN_LAYER_CONFIG")
        .cloned()
        .unwrap_or_else(|| ".usr/config/layers.conf".to_string());

    let mut role = SessionRole::Guest;
    let mut vm = boot_vm(args.memory_size, PlatformProfile::Core, role);
    apply_layer_config(&mut vm, &layer_cfg);

    println!("JevanKernel VM booted.");
    println!("Session is locked as guest. Run: create-admin <name>");
    println!("Type 'help' for commands.");

    repl_loop(&mut vm, args, &mut role, logger, env, layer_cfg)
}

fn repl_loop(
    vm: &mut Vm,
    args: Args,
    role: &mut SessionRole,
    logger: &mut dyn Logger,
    env: HashMap<String, String>,
    layer_cfg: String,
) -> Result<()> {
    let stdin = io::stdin();

    loop {
        print!("jevan-kernel> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        if !dispatch(vm, &args, role, logger, input, &env, &layer_cfg)? {
            break;
        }
    }

    Ok(())
}

fn dispatch(
    vm: &mut Vm,
    args: &Args,
    role: &mut SessionRole,
    logger: &mut dyn Logger,
    input: &str,
    env: &HashMap<String, String>,
    layer_cfg: &str,
) -> Result<bool> {
    let mut parts = input.split_whitespace();
    let cmd = parts.next().unwrap_or_default();
    let argsv: Vec<&str> = parts.collect();

    match cmd {
        "help" => commands::print_help(),
        "env-show" => commands::print_env(env),
        "create-admin" => {
            if matches!(role, SessionRole::Guest) {
                *role = SessionRole::Admin;
                *vm = boot_vm(args.memory_size, PlatformProfile::Core, *role);
                apply_layer_config(vm, layer_cfg);
                println!("admin created and session unlocked");
            } else {
                println!("admin already created for this session");
            }
        }
        "exit" | "quit" => return Ok(false),
        _ if matches!(role, SessionRole::Guest) => println!("guest locked: use create-admin <name> first"),
        "run-demo" => if let Err(e) = commands::run_demo(vm, logger) { println!("error: {e}"); },
        "run" => if let Some(path) = argsv.first() {
            if let Err(e) = commands::run_bytecode(vm, path, logger) { println!("error: {e}"); }
        } else { println!("usage: run <bytecode_file.vmb>"); },
        "status" => commands::print_status(vm),
        "layers" => commands::print_layers(vm),
        "layer-set" => if argsv.len() >= 2 {
            commands::layer_set(vm, argsv[0], argsv[1].eq_ignore_ascii_case("on"));
        } else { println!("usage: layer-set <name> <on|off>"); },
        "mem" => if let Some(raw) = argsv.first() {
            match raw.parse::<usize>() { Ok(a) => commands::print_mem(vm, a), Err(_) => println!("usage: mem <addr>") }
        } else { println!("usage: mem <addr>"); },
        "ls" => if let Err(e) = commands::syscall_ls(vm, argsv.first().copied(), logger) { println!("error: {e}"); },
        "cat" => if let Some(path) = argsv.first() {
            if let Err(e) = commands::syscall_cat(vm, path, logger) { println!("error: {e}"); }
        } else { println!("usage: cat <file>"); },
        "echo" => if let Err(e) = commands::syscall_echo(vm, &argsv, logger) { println!("error: {e}"); },
        "layer-create" => if argsv.len() >= 2 {
            if let Ok(p) = argsv[1].parse::<i32>() {
                if let Err(e) = commands::syscall_create_layer(vm, argsv[0], p, logger) { println!("error: {e}"); }
            } else { println!("usage: layer-create <name> <priority>"); }
        } else { println!("usage: layer-create <name> <priority>"); },
        _ => println!("unknown command: {cmd}"),
    }

    Ok(true)
}

fn apply_layer_config(vm: &mut Vm, path: &str) {
    let config = load_layer_config(path);
    for (name, enabled) in config {
        vm.layers_mut().set_enabled(&name, enabled);
    }
}
