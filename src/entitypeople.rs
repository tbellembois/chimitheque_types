use serde::{Deserialize, Serialize};

/// A manager of an entity, as stored in the `entitypeople` table.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entitypeople {
    /// Id of the managed entity.
    pub entitypeople_entity_id: u64,
    /// Id of the manager.
    pub entitypeople_person_id: u64,

    // Convenient fields not in DB to have all of the necessary informations in the struct.
    /// Email of the manager (not stored in this table).
    pub entitypeople_person_email: String,
    /// Name of the entity (not stored in this table).
    pub entitypeople_entity_name: String,
}

#[cfg(test)]
#[path = "entitypeople_tests.rs"]
mod entitypeople_tests;
