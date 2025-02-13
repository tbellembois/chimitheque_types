use serde::{Deserialize, Serialize};

use crate::person::Person;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub entity_id: u64,
    pub entity_name: String,
    pub entity_description: Option<String>,

    // Computed fields on select, not in DB.
    pub managers: Option<Vec<Person>>,
    pub entity_nb_store_locations: Option<u64>,
    pub entity_nb_people: Option<u64>,
}
