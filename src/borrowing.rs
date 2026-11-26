use serde::{Deserialize, Serialize};

use crate::person::Person;
use crate::{error::ValidationError, validation::clean_optional_text};

/// A storage lent to a person (the borrower).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Borrowing {
    /// Database id; `None` for a new borrowing.
    pub borrowing_id: Option<u64>,
    /// Optional free comment.
    pub borrowing_comment: Option<String>,
    /// Id of the person who lent the storage.
    pub person: u64,
    /// Id of the borrowed storage.
    pub storage: u64,
    /// Person who borrows the storage.
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
        clean_optional_text(&mut self.borrowing_comment);
        Ok(())
    }
}

#[cfg(test)]
#[path = "borrowing_tests.rs"]
mod borrowing_tests;
