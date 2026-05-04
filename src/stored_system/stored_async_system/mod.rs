pub mod async_system;

use crate::prelude::AsyncSystem;

pub type StoredAsyncSystem = Box<dyn AsyncSystem>;