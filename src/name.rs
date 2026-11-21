use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A product name or synonym, stored uppercase.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Name {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new name.
    pub name_id: Option<u64>,
    /// Name, uppercase.
    pub name_label: String,
}

impl Name {
    /// Cleans the name, converts it to uppercase and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned name is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        self.name_label = clean(&self.name_label, Transform::Uppercase);
        if self.name_label.is_empty() {
            return Err(ValidationError::Empty {
                field: "name_label",
            });
        }
        Ok(())
    }
}

impl_searchable!(Name, table: "name", id: name_id, text: name_label);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_label)
    }
}

#[cfg(test)]
#[path = "name_tests.rs"]
mod name_tests;
