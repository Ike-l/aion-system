use aion_program::prelude::{AccessBuilder, ProgramId, UserId, UserPassword, ValuePassword};

#[derive(Clone, Default)]
pub struct ProgramDetails {
    system_program: Option<ProgramId>,
    system_program_password: Option<ValuePassword>,
    system_user_details: Option<(UserId, UserPassword)>,
}

impl ProgramDetails {
    pub fn into_access_builder(self) -> AccessBuilder {
        AccessBuilder {
            program_id: self.system_program,
            program_password: self.system_program_password,
            user_details: self.system_user_details,
            ..Default::default()
        }
    }

    pub fn get_system_program(&self) -> &Option<ProgramId> {
        &self.system_program
    }
}