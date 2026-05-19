use std::collections::HashMap;

use anyhow::Result;
use vm_core::{
    BytecodeFile, Instruction, Logger, Program, Vm, SYSCALL_CAT, SYSCALL_CREATE_LAYER,
    SYSCALL_ECHO, SYSCALL_LS,
};

use crate::programs::sample_program;

pub fn print_env(env: &HashMap<String, String>) {
    let mut keys: Vec<_> = env.keys().collect();
    keys.sort();
    for key in keys {
        if let Some(value) = env.get(key) {
            println!("{key}={value}");
        }
    }
}

pub fn run_demo(vm: &mut Vm, logger: &mut dyn Logger) -> Result<()> {
    vm.reset();
    vm.run(&sample_program(), logger)?;
    print_run_result(vm);
    Ok(())
}

pub fn run_bytecode(vm: &mut Vm, path: &str, logger: &mut dyn Logger) -> Result<()> {
    vm.reset();
    let bytes = std::fs::read(path)?;
    let program = BytecodeFile::decode(&bytes)?;
    vm.run(&program, logger)?;
    print_run_result(vm);
    Ok(())
}

pub fn print_status(vm: &Vm) {
    let regs = vm.registers();
    println!("pc={} acc={} mem_size={}", regs.pc, regs.acc, vm.memory().len());
}

pub fn print_layers(vm: &mut Vm) {
    for (name, priority, enabled) in vm.layers_mut().order() {
        println!("layer name={name} priority={priority} enabled={enabled}");
    }
}

pub fn layer_set(vm: &mut Vm, name: &str, enabled: bool) {
    vm.layers_mut().set_enabled(name, enabled);
}

pub fn print_mem(vm: &Vm, addr: usize) {
    match vm.memory().read(addr) {
        Ok(v) => println!("mem[{addr}]={v}"),
        Err(e) => println!("error: {e}"),
    }
}

pub fn syscall_ls(vm: &mut Vm, path: Option<&str>, logger: &mut dyn Logger) -> Result<()> {
    vm.reset();
    vm.set_syscall_args(vec![path.unwrap_or(".").to_string()]);
    vm.run(&Program::new(vec![Instruction::Syscall(SYSCALL_LS), Instruction::Halt]), logger)?;
    print_run_result(vm);
    Ok(())
}

pub fn syscall_cat(vm: &mut Vm, path: &str, logger: &mut dyn Logger) -> Result<()> {
    vm.reset();
    vm.set_syscall_args(vec![path.to_string()]);
    vm.run(&Program::new(vec![Instruction::Syscall(SYSCALL_CAT), Instruction::Halt]), logger)?;
    print_run_result(vm);
    Ok(())
}

pub fn syscall_echo(vm: &mut Vm, parts: &[&str], logger: &mut dyn Logger) -> Result<()> {
    vm.reset();
    vm.set_syscall_args(vec![parts.join(" ")]);
    vm.run(&Program::new(vec![Instruction::Syscall(SYSCALL_ECHO), Instruction::Halt]), logger)?;
    print_run_result(vm);
    Ok(())
}

pub fn syscall_create_layer(vm: &mut Vm, name: &str, priority: i32, logger: &mut dyn Logger) -> Result<()> {
    vm.reset();
    vm.set_syscall_args(vec![name.to_string(), priority.to_string()]);
    vm.run(&Program::new(vec![Instruction::Syscall(SYSCALL_CREATE_LAYER), Instruction::Halt]), logger)?;
    print_run_result(vm);
    Ok(())
}

pub fn print_run_result(vm: &Vm) {
    for line in vm.output() {
        println!("{line}");
    }
    let regs = vm.registers();
    println!("pc={} acc={}", regs.pc, regs.acc);
}

pub fn print_help() {
    println!("help                                         show this help");
    println!("env                                          show loaded env config");
    println!("account create <name> <admin|user|guest>    first account auto-admin; later admin-only");
    println!("login <name>                                 restore/open session (password prompt)");
    println!("logout                                       close session (password prompt if required)");
    println!("settings show                                display current-user settings");
    println!("settings set session-ttl <secs>              set session timeout");
    println!("settings set logout-password <on|off>        require password to logout");
    println!("demo                                         run built-in demo bytecode");
    println!("run <file.vmb>                               run bytecode file");
    println!("status                                       show vm status");
    println!("layers                                       list layers + active state");
    println!("layer set <name> <on|off>                    toggle layer");
    println!("mklayer <name> <priority>                    create user layer via syscall");
    println!("mem <addr>                                   inspect memory cell");
    println!("ls [dir] | cat <file> | echo <text...>       filesystem/text syscalls");
    println!("exit|quit                                    leave shell");
}
