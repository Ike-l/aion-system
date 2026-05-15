use crate::prelude::{AsyncSystem, SyncSystem};

pub mod async_system;
pub mod sync_system;

pub mod program_details;

pub mod system_result;
pub mod system_error;

pub mod function_system;

pub enum SystemKind {
    Async(AsyncSystem),
    Sync(SyncSystem),
}
pub struct System {
    kind: Option<SystemKind>
}

impl System {
    pub fn take_system(&mut self) -> Option<SystemKind> {
        self.kind.take()
    }

    pub fn put_system(&mut self, system: SystemKind) -> Option<SystemKind> {
        self.kind.replace(system)
    }
}

// System:
// AsyncSystem
// SyncSystem

// Enabled:
// bool

// EventTrigger

// RequiresMainThreadFlag

// ManualAccessBuilders

// IsExecuting