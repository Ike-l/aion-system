pub use aion_program::prelude::{ProgramRegistry};

use crate::prelude::{StoredSystemKind, StoredSystemMetadata};

pub mod stored_sync_system;
pub mod stored_async_system;

pub mod stored_system_kind;
pub mod stored_system_metadata;

pub mod system_result;
pub mod system_id;
/* 
SystemResult
SystemId

FunctionSystemBase

*/

pub struct StoredSystem {
    kind: StoredSystemKind,
    metadata: StoredSystemMetadata
}

