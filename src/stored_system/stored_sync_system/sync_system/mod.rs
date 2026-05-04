use crate::prelude::{SystemId};

pub trait SyncSystem: Send + Sync {
    fn execute(&mut self, system_id: &SystemId);
}