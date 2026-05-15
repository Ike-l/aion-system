pub mod system;

pub mod prelude {
    pub use super::{
        system::{
            System,
            program_details::{
                ProgramDetails
            },
            async_system::{
                AsyncSystem,
                async_system_executable::{
                    AsyncSystemExecutable
                },
                into_async_system::{
                    IntoAsyncSystem
                }
            },
            sync_system::{
                SyncSystem,
                sync_system_executable::{
                    SyncSystemExecutable
                },
                into_sync_system::{
                    IntoSyncSystem
                }
            },
            function_system::{
                function_system_base::{
                    FunctionSystemBase
                }
            },
            system_error::{
                SystemError
            },
            system_result::{
                SystemResult
            }
        }
    };
}
