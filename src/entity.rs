//! Entity module containing the Entity struct and its implementations.
//! Represents organizations, departments, or groups that manage chemical products and storage.
//! Entities can contain multiple people, storage locations, and products.

use std::fmt;

use chimitheque_utils::string::{clean, Transform};
use serde::{Deserialize, Serialize};

use crate::person::Person;

/// Main Entity structure representing an organization or group
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entity {
    /// Unique identifier for the entity
    pub entity_id: Option<u64>,
    /// Name of the entity (organization, department, etc.)
    pub entity_name: String,
    /// Optional description of the entity's purpose or function
    pub entity_description: Option<String>,

    // Computed fields on select, not in DB.
    /// List of managers associated with this entity
    // Managers can contain only one Person
    // with a comma separated list of managers person_email.
    // In this case person_id is populated with the default value.
    pub managers: Option<Vec<Person>>,
    /// Number of storage locations within this entity
    pub entity_nb_store_locations: Option<u64>,
    /// Number of people associated with this entity
    pub entity_nb_people: Option<u64>,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Entity {{ entity_id: {:?}, entity_name: {}, entity_description: {:?} }}",
            self.entity_id, self.entity_name, self.entity_description
        )
    }
}

impl Entity {
    /// Validates and sanitizes all fields of the Entity
    /// Cleans the entity name and description to remove unwanted characters
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.entity_name = clean(&self.entity_name, Transform::None);

        if let Some(entity_description) = self.entity_description.clone() {
            self.entity_description = Some(clean(entity_description.as_str(), Transform::None));
        }
        Ok(())
    }
}
