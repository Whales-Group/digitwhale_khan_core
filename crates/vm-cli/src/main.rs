mod cli;
mod commands;
mod config;
mod env_bootstrap;
mod programs;
mod shell;

use std::fs;

use anyhow::Result;
use clap::Parser;
use vm_core::{boot_vm, BytecodeFile, PlatformProfile, SessionRole, StdoutLogger};

use crate::cli::Args;
use crate::commands::print_run_result;
use crate::env_bootstrap::ensure_env_and_usr;
use crate::programs::{load_program, sample_program};

fn main() -> Result<()> {
    let _env = ensure_env_and_usr()?;

    let args = Args::parse();
    let mut logger = StdoutLogger::new(args.log_level.into());

    if let Some(path) = args.dump_sample_bytecode {
        let bytes = BytecodeFile::encode(&sample_program());
        fs::write(path, bytes)?;
        return Ok(());
    }

    if args.once {
        let mut vm = boot_vm(args.memory_size, PlatformProfile::Core, SessionRole::Admin);
        let program = load_program(args.bytecode)?;
        vm.run(&program, &mut logger)?;
        print_run_result(&vm);
        return Ok(());
    }

    shell::run_shell(args, &mut logger)
}
