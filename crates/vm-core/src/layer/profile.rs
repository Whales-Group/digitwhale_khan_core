use super::{
    CapabilityLayer, LayerOrchestrator, LoggerLayer, MemoryRegion, PlatformIdentityLayer,
    SecurityLayer,
};

#[derive(Debug, Clone, Copy)]
pub enum PlatformProfile {
    Core,
}

pub fn apply_platform_profile(
    orchestrator: &mut LayerOrchestrator,
    profile: PlatformProfile,
    memory_size: usize,
) {
    let max = memory_size.saturating_sub(1);

    match profile {
        PlatformProfile::Core => {
            orchestrator.add_system(PlatformIdentityLayer::new("core"), 10);
            orchestrator.add_system(LoggerLayer, 20);
            orchestrator.add_system(SecurityLayer::new(max), 100);
            orchestrator.add_system(
                CapabilityLayer::new(
                    vec![MemoryRegion {
                        start: 0,
                        end_inclusive: max,
                    }],
                    [1, 2, 10, 11, 12, 20, 30, 40],
                ),
                200,
            );
        }
    }
}
