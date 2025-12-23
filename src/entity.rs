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

impl Entity {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.entity_name = clean(&self.entity_name, Transform::None);

        if self.entity_description.is_some() {
            self.entity_description = Some(clean(
                self.entity_description.as_mut().unwrap(),
                Transform::None,
            ));
        }
        Ok(())
    }
}
