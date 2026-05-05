pub mod system_criteria;

use aion_event::prelude::CurrentEvents;
use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword};

use crate::prelude::{SystemCriteria};

pub struct StoredSystemMetadata {
    criteria: SystemCriteria,
    program_password: Option<ValuePassword>,

    resource_id: ResourceId,

    user_details: Option<(UserId, UserPassword)>,

    // will also check the system at runtime
    readonly: bool,
}

impl Clone for StoredSystemMetadata {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl StoredSystemMetadata {
    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }

    pub fn program_password(&self) -> &Option<ValuePassword> {
        &self.program_password
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }

    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    pub fn test_events(&self, events: &CurrentEvents) -> bool {
        self.criteria.test(events)
    }
}