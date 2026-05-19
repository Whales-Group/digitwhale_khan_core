use thiserror::Error;

#[derive(Debug, Error)]
pub enum BytecodeError {
    #[error("bytecode is too short")]
    TooShort,
    #[error("invalid magic header")]
    InvalidMagic,
    #[error("unsupported version {0}")]
    UnsupportedVersion(u8),
    #[error("truncated bytecode stream")]
    Truncated,
    #[error("invalid opcode {0:#04x}")]
    InvalidOpcode(u8),
}
