use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword, ProgramId, AccessBuilder};

// rename to executable metadata?
#[derive(Debug, Clone)]
pub struct StoredSystemMetadata {
    system_program_password: Option<ValuePassword>,

    system_resource_id: ResourceId,
    system_resource_password: Option<ValuePassword>,

    user_details: Option<(UserId, UserPassword)>,

    stored_access_builders: Vec<AccessBuilder>,

    requires_main_thread: bool,
}

impl StoredSystemMetadata {
    pub fn requires_main_thread(&self) -> bool {
        self.requires_main_thread
    }

    pub fn system_resource_id(&self) -> &ResourceId {
        &self.system_resource_id
    }

    pub fn system_resource_password(&self) -> &Option<ValuePassword> {
        &self.system_resource_password
    }

    pub fn stored_access_builders(&self) -> &Vec<AccessBuilder> {
        &self.stored_access_builders
    }

    pub fn system_program_password(&self) -> &Option<ValuePassword> {
        &self.system_program_password
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }

    pub fn insert_access_builder(&mut self, access_builder: AccessBuilder) {
        self.stored_access_builders.push(access_builder);
    }

    pub fn get_builders(
        &self,
        program_id: ProgramId,
    ) -> (AccessBuilder, Vec<AccessBuilder>) {
        let auto_access_builder = AccessBuilder {
            program_id: Some(program_id),
            program_password: self.system_program_password().clone(),
            user_details: self.user_details.clone(),

            resource_id: None,
            resource_access: None,
            resource_password: None,
        };

        (auto_access_builder, self.stored_access_builders.clone())
    }
}