use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entitypeople {
    pub entitypeople_entity_id: u64,
    pub entitypeople_person_id: u64,

    // Convenient fields not in DB to have all of the necessary informations in the struct.
    pub entitypeople_person_email: String,
    pub entitypeople_entity_name: String,
}
