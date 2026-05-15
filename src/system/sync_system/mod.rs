use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use hecs::Entity;

use crate::prelude::{ProgramDetails, SyncSystemExecutable};

pub mod sync_system_executable;
pub mod into_sync_system;

pub struct SyncSystem {
    executable: Box<dyn SyncSystemExecutable>
}

impl SyncSystem {
    pub fn check_accesses(
        &self, 
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        program_details: &ProgramDetails,
        manual_access_builders: Vec<&AccessBuilder>
    ) -> bool {
        self.executable.check_accesses(system_entity, program_registry, program_details, manual_access_builders)
    }
}