use std::cell::UnsafeCell;

use crate::prelude::StoredSystemKind;

pub struct SystemCell {
    system: UnsafeCell<StoredSystemKind>
}

impl SystemCell {
    pub fn new(system: StoredSystemKind) -> Self {
        Self { system: UnsafeCell::new(system) }
    }

    pub fn consume(self) -> StoredSystemKind {
        self.system.into_inner()
    }

    /// # Safety
    /// 
    /// Ensure Aliasing Rules are not violated
    pub unsafe fn get(&self) -> &mut StoredSystemKind {
        unsafe { &mut *self.system.get() }
    }
}

/// # Safety
/// 
/// Functions are always Send
/// 
/// -Closures aren't 
unsafe impl Send for SystemCell {}

/// # Safety
/// 
/// Functions are always Sync
/// 
/// -Closures aren't 
unsafe impl Sync for SystemCell {}