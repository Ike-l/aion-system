use std::sync::Arc;

use crate::prelude::{ProgramDetails, SystemError, SystemResult};

use aion_program::prelude::{ProgramRegistry, AccessBuilder};
use hecs::Entity;

pub trait SyncSystemExecutable: Send + Sync {
    fn execute(
        &mut self, 
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        program_details: &ProgramDetails,
        manual_access_builders: Vec<&AccessBuilder>,
    ) -> Result<Option<SystemResult>, SystemError>;

    fn check_accesses(
        &self,
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        program_details: &ProgramDetails,
        manual_access_builders: Vec<&AccessBuilder>,
    ) -> bool;
}
