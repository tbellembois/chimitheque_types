use serde::{Deserialize, Serialize};

use crate::person::Person;
use crate::{error::ValidationError, validation::trim_optional_text};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Borrowing {
    pub borrowing_id: Option<u64>,
    pub borrowing_comment: Option<String>,
    pub person: u64,
    pub storage: u64,
    pub borrower: Person,
}

impl Borrowing {
    /// Trims the comment, keeping its line breaks. An empty comment becomes
    /// `None`.
    ///
    /// # Errors
    ///
    /// Never fails today; returns a `Result` like the other types.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        trim_optional_text(&mut self.borrowing_comment);
        Ok(())
    }
}

#[cfg(test)]
#[path = "borrowing_tests.rs"]
mod borrowing_tests;
