use serde::{Deserialize, Serialize};

/// A person belonging to an entity, as stored in the `personentities` table.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Personentities {
    /// Id of the person.
    pub personentities_person_id: u64,
    /// Id of the entity the person belongs to.
    pub personentities_entity_id: u64,

    // Convenient fields not in DB to have all of the necessary informations in the struct.
    /// Email of the person (not stored in this table).
    pub personentities_person_email: String,
    /// Name of the entity (not stored in this table).
    pub personentities_entity_name: String,
}

#[cfg(test)]
#[path = "personentities_tests.rs"]
mod personentities_tests;
