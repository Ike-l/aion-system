pub mod system_criteria;

use aion_program::prelude::{ResourceId, ValuePassword, UserId, UserPassword};

use crate::prelude::{SystemCriteria};

pub struct StoredSystemMetadata {
    criteria: SystemCriteria,
    resource_id: ResourceId,
    program_password: Option<ValuePassword>,
    system_details: Option<(UserId, UserPassword)>
}