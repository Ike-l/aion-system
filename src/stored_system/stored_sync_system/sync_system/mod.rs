use std::sync::Arc;

use crate::prelude::{SystemResult, SystemError};

use aion_program::prelude::{ProgramRegistry, ProgramId, ValuePassword, UserId, UserPassword};

pub mod into_sync_system;

pub trait SyncSystem: Send + Sync {
    fn execute(
        &mut self, 
        program_registry: &Arc<ProgramRegistry>,
        program_id: &ProgramId,
        program_password: Option<&ValuePassword>,
        user_details: Option<(&UserId, &UserPassword)>
    ) -> Result<Option<SystemResult>, SystemError>;
}

