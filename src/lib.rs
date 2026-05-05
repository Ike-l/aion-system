pub mod stored_system;

pub mod prelude {
    pub use super::{
        stored_system::{
            StoredSystem,
            system_result::{
                SystemResult,
            },
            system_error::{
                SystemError
            },
            stored_system_kind::{
                StoredSystemKind
            },
            stored_system_metadata::{
                StoredSystemMetadata,
                stored_access_builder::{
                    StoredAccessBuilder
                }
            },
            function_system_base::{
                FunctionSystemBase
            },
            stored_async_system::{
                StoredAsyncSystem,
                async_system::{
                    AsyncSystem,
                    into_async_system::{
                        IntoAsyncSystem   
                    }
                }
            },
            stored_sync_system::{
                StoredSyncSystem,
                sync_system::{
                    SyncSystem,
                    into_sync_system::{
                        IntoSyncSystem,
                    },
                },
            }
        }
    };
}
