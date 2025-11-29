use std::error::Error;

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

impl Person {
    pub fn is_valid(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let mayerr_parse = EmailAddress::parse_with_options(
            &self.person_email,
            Options {
                ..Default::default()
            },
        );
        match mayerr_parse {
            Ok(_) => Ok(true),
            Err(err) => Err(Box::new(err)),
        }
    }
}
