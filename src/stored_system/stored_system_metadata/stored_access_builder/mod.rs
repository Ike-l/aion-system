use aion_program::prelude::*;

#[derive(Debug, Clone)]
pub struct StoredAccessBuilder {
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub resource_id: Option<ResourceId>,
    pub resource_access: Option<ResourceAccess>,
    pub resource_password: Option<ValuePassword>
}