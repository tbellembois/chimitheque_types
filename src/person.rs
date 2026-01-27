use std::fmt;

use chimitheque_utils::string::{clean, Transform};
use email_address::{EmailAddress, Options};
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

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.person_id == other.person_id && self.person_email == other.person_email
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Person {{ person_id: {:?}, person_email: {} }}",
            self.person_id, self.person_email
        )
    }
}

impl Person {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.person_email = clean(&self.person_email, Transform::Lowercase);

        if let Err(err) = EmailAddress::parse_with_options(
            &self.person_email,
            Options {
                ..Default::default()
            },
        ) {
            Err(Box::new(err))
        } else {
            Ok(())
        }
    }
}
