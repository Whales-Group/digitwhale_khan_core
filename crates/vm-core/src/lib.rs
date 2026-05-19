mod bytecode;
mod layer;
mod logging;
mod memory;
mod runtime;
mod scheduler;

pub use bytecode::{BytecodeError, BytecodeFile, Program};
pub use layer::{
    apply_platform_profile, boot_vm, boot_with_orchestrator, CapabilityLayer, Layer, LayerError,
    LayerEvent, LayerOrchestrator, LoggerLayer, MemoryRegion, PlatformProfile,
    PlatformIdentityLayer, SecurityLayer, SessionRole, UserLayer,
};
pub use logging::{LogEvent, LogLevel, Logger, StdoutLogger, VecLogger};
pub use memory::{Memory, MemoryError};
pub use runtime::{
    Instruction, Registers, Vm, VmConfig, VmError, SYSCALL_CAT, SYSCALL_CREATE_LAYER,
    SYSCALL_DEVICE, SYSCALL_ECHO, SYSCALL_LS, SYSCALL_PRINT, SYSCALL_PROC,
};
pub use scheduler::{RoundRobinScheduler, VmTask};
