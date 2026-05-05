use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword, ResourceAccess};

// rename to executable metadata?

#[derive(Debug, Clone)]
pub struct StoredSystemMetadata {
    program_password: Option<ValuePassword>,

    system_resource_id: ResourceId,
    system_resource_password: Option<ValuePassword>,

    user_details: Option<(UserId, UserPassword)>,

    resource_ids: Vec<ResourceId>,
    resource_passwords: Vec<ValuePassword>,
    resource_accesses: Vec<ResourceAccess>
}

impl StoredSystemMetadata {
    pub fn system_resource_id(&self) -> &ResourceId {
        &self.system_resource_id
    }

    pub fn resource_ids(&self) -> &Vec<ResourceId> {
        &self.resource_ids
    }

    pub fn resource_passwords(&self) -> &Vec<ValuePassword> {
        &self.resource_passwords
    }

    pub fn resource_accesses(&self) -> &Vec<ResourceAccess> {
        &self.resource_accesses
    }

    pub fn system_resource_password(&self) -> &Option<ValuePassword> {
        &self.system_resource_password
    }

    pub fn program_password(&self) -> &Option<ValuePassword> {
        &self.program_password
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }
}