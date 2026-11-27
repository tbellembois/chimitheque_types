use chimitheque_utils::string::{Transform, clean_required_text};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A product category, e.g. a type of laboratory consumable.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new category.
    pub category_id: Option<u64>,
    /// Category label.
    pub category_label: String,
}

impl Category {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        self.category_label = clean_required_text(&self.category_label, Transform::None).ok_or(
            ValidationError::Empty {
                field: "category_label",
            },
        )?;
        Ok(())
    }
}

impl_searchable!(Category, table: "category", id: category_id, text: category_label);

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.category_label)
    }
}

#[cfg(test)]
#[path = "category_tests.rs"]
mod category_tests;
