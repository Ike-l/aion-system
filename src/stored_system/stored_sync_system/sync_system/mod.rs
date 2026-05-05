use std::sync::Arc;

use crate::prelude::{SystemResult, SystemError};

use aion_program::prelude::{ProgramRegistry, AccessBuilder};

pub mod into_sync_system;

pub trait SyncSystem: Send + Sync {
    fn execute(
        &mut self, 
        program_registry: &Arc<ProgramRegistry>,
        auto_access_builder: &AccessBuilder,
        manual_access_builders: Vec<&AccessBuilder>,
    ) -> Result<Option<SystemResult>, SystemError>;
}

