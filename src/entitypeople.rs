use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entitypeople {
    pub entitypeople_entity_id: u64,
    pub entitypeople_person_id: u64,
    pub entitypeople_person_email: String,
}
