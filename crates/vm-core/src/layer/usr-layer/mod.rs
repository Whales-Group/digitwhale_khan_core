use crate::Memory;

use crate::layer::{Layer, LayerError, LayerEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRole {
    Admin,
    User,
    Guest,
}

impl SessionRole {
    pub fn can_read(self) -> bool {
        !matches!(self, SessionRole::Guest)
    }

    pub fn can_write(self) -> bool {
        matches!(self, SessionRole::Admin)
    }
}

pub struct UserLayer {
    role: SessionRole,
}

impl UserLayer {
    pub fn new(role: SessionRole) -> Self {
        Self { role }
    }
}

impl Layer for UserLayer {
    fn name(&self) -> &'static str {
        "usr"
    }

    fn on_event(&mut self, event: &LayerEvent, _memory: &Memory) -> Result<(), LayerError> {
        let denied = match event {
            LayerEvent::BeforeMemoryRead { .. } => !self.role.can_read(),
            LayerEvent::BeforeMemoryWrite { .. } => !self.role.can_write(),
            LayerEvent::BeforeSyscall { syscall_id } => syscall_denied(self.role, *syscall_id),
            LayerEvent::BeforeInstruction { .. } => false,
        };

        if denied {
            Err(LayerError::Blocked {
                layer: self.name(),
                reason: "permission denied".to_string(),
            })
        } else {
            Ok(())
        }
    }
}

fn syscall_denied(role: SessionRole, syscall_id: u16) -> bool {
    if role == SessionRole::Admin {
        return false;
    }

    // Write syscall(s): layer mutation.
    if syscall_id == 40 {
        return true;
    }

    // Read-only user can use read syscalls.
    if role == SessionRole::User {
        return false;
    }

    // Guest blocked from all syscalls.
    true
}
