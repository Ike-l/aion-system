use crate::prelude::{AsyncSystem, SyncSystem};

pub mod async_system;
pub mod sync_system;

pub mod program_details;

pub mod system_result;
pub mod system_error;

pub mod function_system;

pub enum System {
    Async(AsyncSystem),
    Sync(SyncSystem),
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