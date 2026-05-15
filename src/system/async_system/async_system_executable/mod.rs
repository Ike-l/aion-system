use std::{pin::Pin, sync::Arc};

use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use hecs::Entity;

use crate::prelude::{ProgramDetails, SystemError, SystemResult};

pub trait AsyncSystemExecutable: Send + Sync {
    fn execute<'a>(
        &'a mut self,
        system_entity: Entity,
        program_registry: Arc<ProgramRegistry>,
        program_details: ProgramDetails,
        manual_access_builders: Vec<AccessBuilder>,
    ) -> Pin<Box<dyn Future<Output = Result<Option<SystemResult>, SystemError>> + 'a + Send>>;

    fn check_accesses(
        &self,
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        program_details: &ProgramDetails,
        manual_access_builders: Vec<&AccessBuilder>,
    ) -> bool;
}