pub mod stored_system;

pub mod prelude {
    pub use super::{
        stored_system::{
            StoredSystem,
            system_id::{
                SystemId
            },
            stored_system_kind::{
                StoredSystemKind
            },
            stored_system_metadata::{
                StoredSystemMetadata,
                system_criteria::{
                    SystemCriteria
                }
            },
            stored_async_system::{
                StoredAsyncSystem
            },
            stored_sync_system::{
                StoredSyncSystem,
                sync_system::{
                    SyncSystem
                }
            }
        }
    };
}
