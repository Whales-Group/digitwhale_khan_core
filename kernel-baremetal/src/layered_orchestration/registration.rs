use super::LayerStage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerKind {
    System,
    User,
}

#[derive(Clone, Copy)]
pub struct LayerRegistration {
    pub name: &'static str,
    pub kind: LayerKind,
    pub immutable: bool,
    pub enabled: bool,
    pub priority: u16,
    pub stage: LayerStage,
}
