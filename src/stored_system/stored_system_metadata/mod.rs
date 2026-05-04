pub mod system_criteria;

use aion_event::prelude::CurrentEvents;
use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword};

use crate::prelude::{SystemCriteria};

pub struct StoredSystemMetadata {
    criteria: SystemCriteria,
    resource_id: ResourceId,
    program_password: Option<ValuePassword>,
    system_details: Option<(UserId, UserPassword)>,

    // will also check the system at runtime
    readonly: bool,
}

impl StoredSystemMetadata {
    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }

    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    pub fn test_events(&self, events: &CurrentEvents) -> bool {
        self.criteria.test(events)
    }
}