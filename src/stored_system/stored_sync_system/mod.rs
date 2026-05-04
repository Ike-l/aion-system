pub mod sync_system;

use crate::prelude::{SyncSystem};

pub struct StoredSyncSystem {
    system: Box<dyn SyncSystem>
}