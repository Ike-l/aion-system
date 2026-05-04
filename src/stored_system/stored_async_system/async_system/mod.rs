use std::{pin::Pin, sync::Arc};

use crate::prelude::{SystemResult, SystemError};

use aion_program::prelude::{ProgramRegistry, ProgramId, ValuePassword, UserId, UserPassword};

pub mod into_async_system;

pub trait AsyncSystem: Send + Sync {
    fn execute<'a>(
        &'a mut self,
        program_registry: Arc<ProgramRegistry>,
        program_id: ProgramId,
        program_password: Option<ValuePassword>,
        user_details: Option<(UserId, UserPassword)>
    ) -> Pin<Box<dyn Future<Output = Result<Option<SystemResult>, SystemError>> + 'a + Send>>;
}