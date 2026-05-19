use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use vm_core::LogLevel;

#[derive(Parser, Debug, Clone)]
#[command(name = "vm-cli")]
#[command(about = "JevanKernel terminal VM")]
pub struct Args {
    #[arg(long)]
    pub bytecode: Option<PathBuf>,

    #[arg(long)]
    pub dump_sample_bytecode: Option<PathBuf>,

    #[arg(long, default_value_t = 256)]
    pub memory_size: usize,

    #[arg(long, value_enum, default_value_t = CliLogLevel::Info)]
    pub log_level: CliLogLevel,

    #[arg(long, default_value_t = false)]
    pub once: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliLogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

impl From<CliLogLevel> for LogLevel {
    fn from(value: CliLogLevel) -> Self {
        match value {
            CliLogLevel::Trace => LogLevel::Trace,
            CliLogLevel::Info => LogLevel::Info,
            CliLogLevel::Warn => LogLevel::Warn,
            CliLogLevel::Error => LogLevel::Error,
        }
    }
}
