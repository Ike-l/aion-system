use crate::prelude::{StoredAsyncSystem, StoredSyncSystem};

pub enum StoredSystemKind {
    Sync(StoredSyncSystem),
    Async(StoredAsyncSystem)
}
