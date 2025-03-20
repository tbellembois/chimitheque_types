use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Personentities {
    pub personentities_person_id: u64,
    pub personentities_entity_id: u64,

    // Convenient fields not in DB to have all of the necessary informations in the struct.
    pub personentities_person_email: String,
    pub personentities_entity_name: String,
}
