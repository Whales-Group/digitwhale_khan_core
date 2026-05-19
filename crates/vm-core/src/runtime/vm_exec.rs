use crate::{Instruction, LayerEvent, LogEvent, LogLevel, Logger, Program, SessionRole, UserLayer};

use super::syscalls::{handle_syscall, SYSCALL_CREATE_LAYER};
use super::{Vm, VmError};

impl Vm {
    pub fn run(&mut self, program: &Program, logger: &mut dyn Logger) -> Result<(), VmError> {
        while !self.halted {
            self.step(program, logger)?;
        }
        Ok(())
    }

    fn step(&mut self, program: &Program, logger: &mut dyn Logger) -> Result<(), VmError> {
        let ins = program.get(self.regs.pc).cloned().ok_or(VmError::PcOutOfBounds(self.regs.pc))?;
        self.before_instruction(&ins)?;
        self.trace_instruction(&ins, logger);
        let mut pc_changed = false;
        self.execute(ins, logger, &mut pc_changed)?;
        if !pc_changed && !self.halted { self.regs.pc += 1; }
        Ok(())
    }

    fn before_instruction(&mut self, ins: &Instruction) -> Result<(), VmError> {
        self.layers.notify(&LayerEvent::BeforeInstruction { pc: self.regs.pc, instruction: ins.clone() }, &self.memory)?;
        Ok(())
    }

    fn trace_instruction(&self, ins: &Instruction, logger: &mut dyn Logger) {
        logger.log(LogEvent { level: LogLevel::Trace, component: "vm", message: format!("pc={} ins={:?} acc={}", self.regs.pc, ins, self.regs.acc) });
    }

    fn execute(&mut self, ins: Instruction, logger: &mut dyn Logger, pc_changed: &mut bool) -> Result<(), VmError> {
        match ins {
            Instruction::Nop => {}
            Instruction::LoadImm(v) => self.regs.acc = v,
            Instruction::AddImm(v) => self.regs.acc += v,
            Instruction::SubImm(v) => self.regs.acc -= v,
            Instruction::Store(a) => self.mem_write(a, self.regs.acc)?,
            Instruction::Load(a) => self.regs.acc = self.mem_read(a)?,
            Instruction::AddMem(a) => self.regs.acc += self.mem_read(a)?,
            Instruction::Jump(t) => { self.regs.pc = t; *pc_changed = true; }
            Instruction::JumpIfZero(t) => if self.regs.acc == 0 { self.regs.pc = t; *pc_changed = true; },
            Instruction::Syscall(id) => self.syscall(id, logger)?,
            Instruction::Print => self.print_line(format!("ACC={}", self.regs.acc), logger),
            Instruction::Halt => self.halted = true,
        }
        Ok(())
    }

    fn mem_read(&mut self, addr: usize) -> Result<i64, VmError> {
        self.layers.notify(&LayerEvent::BeforeMemoryRead { addr }, &self.memory)?;
        Ok(self.memory.read(addr)?)
    }

    fn mem_write(&mut self, addr: usize, value: i64) -> Result<(), VmError> {
        self.layers.notify(&LayerEvent::BeforeMemoryWrite { addr, value }, &self.memory)?;
        Ok(self.memory.write(addr, value)?)
    }

    fn syscall(&mut self, id: u16, logger: &mut dyn Logger) -> Result<(), VmError> {
        self.layers.notify(
            &LayerEvent::BeforeSyscall {
                syscall_id: id,
                args: self.syscall_args.clone(),
            },
            &self.memory,
        )?;

        let line = if id == SYSCALL_CREATE_LAYER { self.create_layer_from_kernel() }
        else { handle_syscall(id, &self.syscall_args, &self.memory, self.regs.acc) };

        self.print_line(line, logger);
        self.syscall_args.clear();
        Ok(())
    }

    fn create_layer_from_kernel(&mut self) -> String {
        let name = self.syscall_args.first().cloned().unwrap_or_else(|| "usr-dynamic".to_string());
        let priority = self.syscall_args.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(300);
        self.layers.add_user(UserLayer::new(SessionRole::User), priority);
        format!("layer created: name={name} priority={priority}")
    }

    fn print_line(&mut self, line: String, logger: &mut dyn Logger) {
        self.output.push(line.clone());
        logger.log(LogEvent { level: LogLevel::Info, component: "vm", message: line });
    }
}
