use std::sync::Arc;

use crate::prelude::{SystemResult, SystemError};

use aion_program::prelude::{ProgramRegistry, AccessBuilder};
use hecs::Entity;

pub trait SyncSystemExecutable: Send + Sync {
    fn execute(
        &mut self, 
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        auto_access_builder: &AccessBuilder,
        manual_access_builders: Vec<&AccessBuilder>,
    ) -> Result<Option<SystemResult>, SystemError>;

    fn check_accesses(
        &self,
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        auto_access_builder: &AccessBuilder,
        manual_access_builders: Vec<&AccessBuilder>,
    ) -> bool;
}
