use serde::{Deserialize, Serialize};

use crate::entity::Entity;
use crate::permission::Permission;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Person {
    pub person_id: Option<u64>,
    pub person_email: String,

    // Computed fields on select, not in DB.
    pub entities: Option<Vec<Entity>>,
    pub managed_entities: Option<Vec<Entity>>,
    pub permissions: Option<Vec<Permission>>,
    #[serde(default)]
    pub is_admin: bool,
}
