pub mod async_system;

use crate::prelude::AsyncSystem;

pub struct StoredAsyncSystem {
    system: Box<dyn AsyncSystem>
}