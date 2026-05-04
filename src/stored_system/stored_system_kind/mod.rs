use std::sync::Arc;

use crate::prelude::{StoredAsyncSystem, StoredSyncSystem};

use aion_program::prelude::{ProgramRegistry, ProgramId, ValuePassword, UserId, UserPassword};

pub enum StoredSystemKind {
    Sync(StoredSyncSystem),
    Async(StoredAsyncSystem)
}

impl StoredSystemKind {
    pub fn reserve_accesses(
        &self,
        program_registry: &Arc<ProgramRegistry>,
        program_id: &ProgramId,
        program_password: Option<&ValuePassword>,
        user_details: Option<(&UserId, &UserPassword)> 
    ) -> bool {
        match self {
            StoredSystemKind::Sync(sync_system) => {
                sync_system.reserve_accesses(
                    program_registry,
                    program_id,
                    program_password,
                    user_details
                )
            },
            StoredSystemKind::Async(async_system) => {
                async_system.reserve_accesses(
                    program_registry,
                    program_id,
                    program_password,
                    user_details
                )
            },
        }
    }

    pub fn check_read_only(&self) -> bool {
        match self {
            StoredSystemKind::Sync(sync_system) => sync_system.check_read_only(),
            StoredSystemKind::Async(async_system) => async_system.check_read_only(),
        }
    }
}