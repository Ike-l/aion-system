use crate::prelude::{SyncSystem};

pub mod function_system_into_sync_system;
pub mod impl_sync_system_on_function_system;

pub trait IntoSyncSystem<Input> {
    type System: SyncSystem;

    fn into_system(self) -> Self::System;
}