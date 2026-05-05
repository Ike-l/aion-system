use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword, ProgramId, AccessBuilder};

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

    pub fn get_builders<'a>(
        &'a self,
        program_id: &'a ProgramId,
    ) -> (AccessBuilder<'a>, Vec<AccessBuilder<'a>>) {
        let user_details = self.user_details().as_ref().map(|(user_id, user_password)| { (user_id, user_password) });
        let auto_access_builder = AccessBuilder {
            program_id: Some(program_id),
            program_password: self.system_program_password().as_ref(),
            user_details,

            resource_id: None,
            resource_access: None,
            resource_password: None,
        };

        let stored_access_builders = self.stored_access_builders();
        let manual_access_builders: Vec<_> = stored_access_builders.iter().map(|stored_access_builder| {
            AccessBuilder {
                program_id: stored_access_builder.program_id.as_ref(),
                program_password: stored_access_builder.program_password.as_ref(),
                user_details,
                resource_id: stored_access_builder.resource_id.clone(),
                resource_access: stored_access_builder.resource_access.clone(),
                resource_password: stored_access_builder.resource_password.as_ref(),
            }
        }).collect();

        (auto_access_builder, manual_access_builders)
    }
}