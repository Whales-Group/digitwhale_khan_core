#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerError {
    Capacity,
    Duplicate,
    Locked,
    NotFound,
    InitFailed(&'static str),
    HookFailed(&'static str),
}
