pub use aion_program::prelude::{ProgramRegistry};

use crate::prelude::{StoredSystemKind};

pub mod stored_sync_system;
pub mod stored_async_system;
pub mod stored_system_kind;
pub mod stored_system_metadata;
pub mod system_result;
pub mod system_error;
pub mod function_system_base;

pub struct StoredSystem {
    pub kind: Option<StoredSystemKind>,
    pub enabled: bool
}
