use std::sync::{Arc, Mutex};

use crate::prelude::{StoredAsyncSystem, StoredSyncSystem};

pub enum StoredSystemKind {
    Sync(Arc<Mutex<StoredSyncSystem>>),
    Async(Arc<Mutex<StoredAsyncSystem>>)
}