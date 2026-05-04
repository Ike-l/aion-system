use crate::prelude::{SystemId, SystemResult};

pub trait SyncSystem: Send + Sync {
    fn execute(&mut self, system_id: &SystemId) -> Option<SystemResult>;
}