use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use hecs::Entity;

use crate::prelude::{AsyncSystemExecutable, ProgramDetails};

pub mod async_system_executable;
pub mod into_async_system;

pub struct AsyncSystem {
    executable: Option<Box<dyn AsyncSystemExecutable>>
}

type StoredAsyncSystemExecutable = Box<dyn AsyncSystemExecutable>;

impl AsyncSystem {
    pub fn take_system(&mut self) -> Option<StoredAsyncSystemExecutable> {
        self.executable.take()
    }

    pub fn put_system(&mut self, system: StoredAsyncSystemExecutable) -> Option<StoredAsyncSystemExecutable> {
        self.executable.replace(system)
    }

    pub fn check_accesses(
        &self, 
        system_entity: Entity,
        program_registry: &Arc<ProgramRegistry>,
        program_details: &ProgramDetails,
        manual_access_builders: Vec<&AccessBuilder>
    ) -> bool {
        match &self.executable {
            Some(stored_system) => {
                stored_system.check_accesses(system_entity, program_registry, program_details, manual_access_builders)
            },
            None => false,
        }
    }
}