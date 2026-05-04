pub mod async_system;

pub struct StoredAsyncSystem {
    system: Box<dyn AsyncSystem>
}