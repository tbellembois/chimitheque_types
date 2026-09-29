use std::fmt;

use chimitheque_utils::string::{Transform, clean};
use email_address::{EmailAddress, Options};
use serde::{Deserialize, Serialize};

use crate::entity::Entity;
use crate::permission::Permission;

/// Represents a person in the system with their unique identifier and email.
/// Contains computed fields that are populated during selection but not stored in the database.

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Person {
    /// Unique identifier for the person (None if not yet persisted)
    pub person_id: Option<u64>,
    /// Email address of the person (will be sanitized and validated)
    pub person_email: String,

    // Computed fields on select, not in DB.
    /// List of entities this person has access to
    pub entities: Option<Vec<Entity>>,
    /// List of entities this person manages
    pub managed_entities: Option<Vec<Entity>>,
    /// List of permissions this person has
    pub permissions: Option<Vec<Permission>>,
    #[serde(default)]
    /// Flag indicating if this person is an administrator
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
    /// Sanitizes and validates the person's email address.
    /// Converts email to lowercase and validates it's a properly formatted email address.
    ///
    /// # Returns
    /// - `Ok(())` if email is valid
    /// - `Err(Box<dyn std::error::Error + Send + Sync>)` if email is invalid
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

#[cfg(test)]
#[path = "person_tests.rs"]
mod person_tests;
