use crate::{Vm, VmConfig};

use super::{
    apply_platform_profile, EnvLayer, LayerOrchestrator, PlatformProfile, SessionRole, UserLayer,
};

// Central boot path: env first, then usr gate, then system profile stack.
pub fn boot_vm(memory_size: usize, profile: PlatformProfile, role: SessionRole) -> Vm {
    let mut vm = Vm::new(VmConfig { memory_size });
    vm.layers_mut().add_system(EnvLayer::from_dotenv(".env"), -10);
    vm.layers_mut().add_system(UserLayer::new(role), 0);
    apply_platform_profile(vm.layers_mut(), profile, memory_size);
    vm
}

pub fn boot_with_orchestrator(memory_size: usize, orchestrator: LayerOrchestrator) -> Vm {
    let mut vm = Vm::new(VmConfig { memory_size });
    *vm.layers_mut() = orchestrator;
    vm
}
