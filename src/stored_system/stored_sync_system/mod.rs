pub mod sync_system;

use crate::prelude::{SyncSystem};

pub type StoredSyncSystem = Box<dyn SyncSystem>;