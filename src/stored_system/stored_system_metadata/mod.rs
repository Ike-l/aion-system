use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword};

#[derive(Clone)]
pub struct StoredSystemMetadata {
    program_password: Option<ValuePassword>,

    resource_id: ResourceId,

    user_details: Option<(UserId, UserPassword)>,
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
}