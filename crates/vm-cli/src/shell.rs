use std::collections::HashMap;

use anyhow::Result;
use vm_core::{boot_vm, Logger, PlatformProfile, SessionRole, Vm};

use crate::cli::Args;
use crate::config::load_layer_config;
use crate::env_bootstrap::ensure_env_and_usr;
use crate::history::ShellEditor;
use crate::session::{self, SessionState};
use crate::shell_dispatch::dispatch;

pub fn run_shell(args: Args, logger: &mut dyn Logger) -> Result<()> {
    let env = ensure_env_and_usr()?;
    let layer_cfg = env
        .get("JEVAN_LAYER_CONFIG")
        .cloned()
        .unwrap_or_else(|| ".usr/config/layers.conf".to_string());

    let mut session_state = session::restore(&env)?;
    let mut vm = boot_vm(args.memory_size, PlatformProfile::Core, session_state.role);
    apply_layer_config(&mut vm, &layer_cfg);

    println!("JevanKernel VM booted.");
    if matches!(session_state.role, SessionRole::Guest) {
        println!("Session is guest. Create account: account create <name> admin");
        println!("Then login: login <name>");
    } else if let Some(user) = session_state.user.as_deref() {
        println!("Session restored for user '{user}'.");
    }
    println!("Type 'help' for commands.");

    let history_path = env
        .get("JEVAN_HISTORY_FILE")
        .cloned()
        .unwrap_or_else(|| ".usr/history/commands.history".to_string());

    repl_loop(&mut vm, args, &mut session_state, logger, env, layer_cfg, history_path)
}

fn repl_loop(
    vm: &mut Vm,
    args: Args,
    state: &mut SessionState,
    logger: &mut dyn Logger,
    env: HashMap<String, String>,
    layer_cfg: String,
    history_path: String,
) -> Result<()> {
    let mut editor = ShellEditor::new(history_path)?;

    loop {
        let Some(line) = editor.read_line("jevan-kernel> ")? else { break };

        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if should_store_history(input) {
            editor.add_history(input);
        }

        let apply_layers = |vm: &mut Vm| apply_layer_config(vm, &layer_cfg);
        if !dispatch(vm, &args, state, logger, input, &env, apply_layers)? {
            break;
        }
    }

    editor.save()?;
    Ok(())
}

fn apply_layer_config(vm: &mut Vm, path: &str) {
    let config = load_layer_config(path);
    for (name, enabled) in config {
        vm.layers_mut().set_enabled(&name, enabled);
    }
}

fn should_store_history(input: &str) -> bool {
    let cmd = input.split_whitespace().next().unwrap_or_default();
    !matches!(cmd, "login" | "logout" | "account")
}
