use crate::prelude::SyncSystemExecutable;

pub trait IntoSyncSystem<Input> {
    type System: SyncSystemExecutable;

    fn into_system(self) -> Self::System;
}