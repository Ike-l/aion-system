use crate::prelude::{StoredAsyncSystem, StoredSyncSystem, IntoAsyncSystem, IntoSyncSystem, SyncSystem, AsyncSystem};

pub enum StoredSystemKind {
    Sync(StoredSyncSystem),
    Async(StoredAsyncSystem)
}

impl StoredSystemKind {
    pub fn new_sync<T, S, I>(system: T) -> Self where T: IntoSyncSystem<I, System = S>, S: SyncSystem + 'static {
        Self::Sync(Box::new(system.into_system()))
    }

    pub fn new_async<T, S, I>(system: T) -> Self where T: IntoAsyncSystem<I, System = S>, S: AsyncSystem + 'static {
        Self::Async(Box::new(system.into_system()))
    }
}
