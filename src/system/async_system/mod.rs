use std::{pin::Pin, sync::Arc};

use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use hecs::Entity;

use crate::prelude::{AsyncSystemExecutable, ProgramDetails, SystemError, SystemResult};

pub mod async_system_executable;
pub mod into_async_system;

pub struct AsyncSystem {
    executable: Box<dyn AsyncSystemExecutable>
}

impl AsyncSystem {
    pub fn check_accesses(
        &self, 
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        program_details: &ProgramDetails,
        manual_access_builders: Vec<&AccessBuilder>
    ) -> bool {
        self.executable.check_accesses(system_entity, program_registry, program_details, manual_access_builders)
    }

    pub fn execute<'a>(
        &'a mut self,
        system_entity: Entity,
        program_registry: Arc<ProgramRegistry>,
        program_details: ProgramDetails,
        manual_access_builders: Vec<AccessBuilder>
    ) -> Pin<Box<dyn Future<Output = Result<Option<SystemResult>, SystemError>> + 'a + Send>> {
        self.executable.execute(system_entity, program_registry, program_details, manual_access_builders)
    }
}