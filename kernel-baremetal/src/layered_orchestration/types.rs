use super::LayerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerStage {
    Boot,
    Memory,
    Interrupts,
    Drivers,
    Tasking,
}

pub type LayerInit = fn() -> Result<(), LayerError>;
pub type LayerHook = fn(LayerStage) -> Result<(), LayerError>;

#[derive(Clone, Copy)]
pub struct LayerDesc {
    pub name: &'static str,
    pub priority: u16,
    pub locked: bool,
    pub enabled: bool,
    pub stage: LayerStage,
    pub on_init: LayerInit,
    pub on_before: Option<LayerHook>,
    pub on_after: Option<LayerHook>,
}
