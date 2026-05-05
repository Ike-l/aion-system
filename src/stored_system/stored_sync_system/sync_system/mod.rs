use std::sync::Arc;

use crate::prelude::{SystemResult, SystemError};

use aion_program::prelude::{ProgramRegistry, ProgramId, ValuePassword, UserId, UserPassword, ResourceId, ResourceAccess};

pub mod into_sync_system;

pub trait SyncSystem: Send + Sync {
    fn execute(
        &mut self, 
        program_registry: &Arc<ProgramRegistry>,
        program_id: &ProgramId,
        program_password: Option<&ValuePassword>,
        user_details: Option<(&UserId, &UserPassword)>,
        resource_ids: Vec<&ResourceId>,
        resource_passwords: Vec<&ValuePassword>,
        resource_accesses: Vec<&ResourceAccess>
    ) -> Result<Option<SystemResult>, SystemError>;
}

