use std::sync::Arc;

use aion_program::prelude::{AccessBuilder, ProgramRegistry};

use crate::prelude::{StoredAsyncSystem, StoredSyncSystem, IntoAsyncSystem, IntoSyncSystem, SyncSystem, AsyncSystem};

pub enum StoredSystemKind {
    Sync(StoredSyncSystem),
    Async(StoredAsyncSystem)
}

impl StoredSystemKind {
    pub fn new_sync<T, S, I>(system: T) -> Self where T: IntoSyncSystem<I, System = S>, S: SyncSystem + 'static {
        Self::Sync(Box::new(system.into_system()))
    }

    pub fn new_async<T, S, I>(system: T) -> Self where T: IntoAsyncSystem<I, System = S>, S: AsyncSystem + 'static {
        Self::Async(Box::new(system.into_system()))
    }

    pub fn check_access(&self, program_registry: &Arc<ProgramRegistry>, auto_access_builder: &AccessBuilder, manual_access_builders: Vec<&AccessBuilder>) -> bool {
        match self {
            StoredSystemKind::Sync(sync_system) => {
                sync_system.check_accesses(program_registry, auto_access_builder, manual_access_builders)  
            },
            StoredSystemKind::Async(async_system) => {
                async_system.check_accesses(program_registry, auto_access_builder, manual_access_builders)
            },
        }
    }
}
