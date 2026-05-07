use std::{pin::Pin, sync::Arc};

use crate::prelude::{SystemResult, SystemError};

use aion_program::prelude::{ProgramRegistry, AccessBuilder};

pub mod into_async_system;

pub trait AsyncSystem: Send + Sync {
    fn execute<'a>(
        &'a mut self,
        program_registry: Arc<ProgramRegistry>,
        auto_access_builder: AccessBuilder,
        manual_access_builders: Vec<AccessBuilder>,
    ) -> Pin<Box<dyn Future<Output = Result<Option<SystemResult>, SystemError>> + 'a + Send>>;
}