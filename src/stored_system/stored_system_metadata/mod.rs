use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword};

use crate::prelude::StoredAccessBuilder;

pub mod stored_access_builder;

// rename to executable metadata?
#[derive(Debug, Clone)]
pub struct StoredSystemMetadata {
    system_program_password: Option<ValuePassword>,

    system_resource_id: ResourceId,
    system_resource_password: Option<ValuePassword>,

    user_details: Option<(UserId, UserPassword)>,

    stored_access_builders: Vec<StoredAccessBuilder>,
}

impl StoredSystemMetadata {
    pub fn system_resource_id(&self) -> &ResourceId {
        &self.system_resource_id
    }

    pub fn system_resource_password(&self) -> &Option<ValuePassword> {
        &self.system_resource_password
    }

    pub fn stored_access_builders(&self) -> &Vec<StoredAccessBuilder> {
        &self.stored_access_builders
    }

    pub fn system_program_password(&self) -> &Option<ValuePassword> {
        &self.system_program_password
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }
}