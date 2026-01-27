use std::fmt;

use chimitheque_utils::string::{clean, Transform};
use serde::{Deserialize, Serialize};

use crate::person::Person;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entity {
    pub entity_id: Option<u64>,
    pub entity_name: String,
    pub entity_description: Option<String>,

    // Computed fields on select, not in DB.
    // Managers can contain only one Person
    // with a comma separated list of managers person_email.
    // In this case person_id is populated with the default value.
    pub managers: Option<Vec<Person>>,
    pub entity_nb_store_locations: Option<u64>,
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
