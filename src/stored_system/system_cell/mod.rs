use std::cell::UnsafeCell;

use crate::prelude::StoredSystemKind;

pub struct SystemCell {
    system: UnsafeCell<StoredSystemKind>
}

impl SystemCell {
    pub fn new(system: StoredSystemKind) -> Self {
        Self { system: UnsafeCell::new(system) }
    }
}