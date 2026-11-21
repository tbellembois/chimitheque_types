use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A class of chemical compounds, e.g. `alcohol`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassOfCompound {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new class of compound.
    pub class_of_compound_id: Option<u64>,
    /// Class label.
    pub class_of_compound_label: String,
}

impl ClassOfCompound {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        self.class_of_compound_label = clean(&self.class_of_compound_label, Transform::None);
        if self.class_of_compound_label.is_empty() {
            return Err(ValidationError::Empty {
                field: "class_of_compound_label",
            });
        }
        Ok(())
    }
}

impl_searchable!(ClassOfCompound, table: "class_of_compound", id: class_of_compound_id, text: class_of_compound_label);

impl fmt::Display for ClassOfCompound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_of_compound_label)
    }
}

#[cfg(test)]
#[path = "classofcompound_tests.rs"]
mod classofcompound_tests;
