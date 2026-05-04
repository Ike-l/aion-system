pub mod system_criteria;

use aion_program::prelude::ResourceId;

use crate::prelude::{SystemCriteria};

pub struct StoredSystemMetadata {
    criteria: SystemCriteria,
    resource_id: ResourceId
}