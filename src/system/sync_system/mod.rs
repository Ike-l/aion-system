use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use hecs::Entity;

use crate::prelude::SyncSystemExecutable;

pub mod sync_system_executable;
pub mod into_sync_system;

pub struct SyncSystem {
    executable: Option<Box<dyn SyncSystemExecutable>>
}

type StoredSyncSystemExecutable = Box<dyn SyncSystemExecutable>;

impl SyncSystem {
    pub fn take_system(&mut self) -> Option<StoredSyncSystemExecutable> {
        self.executable.take()
    }

    pub fn put_system(&mut self, system: StoredSyncSystemExecutable) -> Option<StoredSyncSystemExecutable> {
        self.executable.replace(system)
    }

    pub fn check_accesses(
        &self, 
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        auto_access_builder: &AccessBuilder,
        manual_access_builders: Vec<&AccessBuilder>
    ) -> bool {
        match &self.executable {
            Some(stored_system) => {
                stored_system.check_accesses(system_entity, program_registry, auto_access_builder, manual_access_builders)
            },
            None => false,
        }
    }
}