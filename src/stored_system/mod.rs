use std::sync::Mutex;

pub use aion_program::prelude::{ProgramRegistry};

use crate::prelude::{StoredSystemKind, SystemStatus, SystemCell};

pub mod stored_sync_system;
pub mod stored_async_system;
pub mod stored_system_kind;
pub mod stored_system_metadata;
pub mod system_result;
pub mod system_error;
pub mod function_system_base;
pub mod system_status;
pub mod system_cell;

pub struct StoredSystem {
    kind: Option<StoredSystemKind>,
    pub status: Mutex<SystemStatus>
}

impl StoredSystem {
    pub fn into_cell(&mut self) -> Option<SystemCell> {
        Some(SystemCell::new(self.kind.take()?))
    }
}

