use std::sync::Arc;

use aion_program::prelude::AccessBuilder;
pub use aion_program::prelude::{ProgramRegistry};

use crate::prelude::{StoredSystemKind};

pub mod stored_sync_system;
pub mod stored_async_system;
pub mod stored_system_kind;
pub mod stored_system_metadata;
pub mod system_result;
pub mod system_error;
pub mod function_system_base;

pub struct StoredSystem {
    kind: Option<StoredSystemKind>,
    enabled: bool
}

impl StoredSystem {
    pub fn take_system(&mut self) -> Option<StoredSystemKind> {
        self.kind.take()
    }

    pub fn put_system(&mut self, system: StoredSystemKind) -> Option<StoredSystemKind> {
        self.kind.replace(system)
    }

    pub fn can_run(
        &self, 
        program_registry: &Arc<ProgramRegistry>,
        auto_access_builder: &AccessBuilder,
        manual_access_builders: Vec<&AccessBuilder>
    ) -> bool {
        if !self.enabled {
            return false;
        }

        match &self.kind {
            Some(stored_system) => {
                stored_system.check_access(program_registry, auto_access_builder, manual_access_builders)
            },
            None => false,
        }
    }
}