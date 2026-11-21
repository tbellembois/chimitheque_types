use email_address::{EmailAddress, Options};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::entity::Entity;
use crate::error::ValidationError;
use crate::permission::Permission;

/// Represents a person in the system with their unique identifier and email.
/// Contains computed fields that are populated during selection but not stored in the database.

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Person {
    /// Unique identifier for the person (None if not yet persisted)
    pub person_id: Option<u64>,
    /// Bare mailbox address; explicitly normalized by `Self::sanitize_and_validate`.
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
    /// Validates a bare mailbox address and applies the account email policy.
    ///
    /// Trims surrounding whitespace and lowercases the entire address, including
    /// the local part, as an application-level account identity policy. Plus tags,
    /// quotes, and quoted whitespace are preserved. Display names and angle-bracket
    /// wrappers are rejected. Single-label domains and domain literals remain
    /// supported. No IDNA conversion or provider-specific alias normalization is
    /// performed.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidEmail`, wrapping the email parser's error,
    /// for invalid or unsupported mailbox syntax.
    /// The original email is left unchanged on failure.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // Lowercase local parts deliberately, but do not collapse quoted whitespace.
        // Validate the normalized candidate before replacing the original value.
        let normalized = self.person_email.trim().to_lowercase();
        EmailAddress::parse_with_options(
            &normalized,
            Options {
                minimum_sub_domains: 0,
                allow_domain_literal: true,
                allow_display_text: false,
            },
        )
        .map_err(ValidationError::InvalidEmail)?;

        self.person_email = normalized;
        Ok(())
    }
}

#[cfg(test)]
#[path = "person_tests.rs"]
mod person_tests;
