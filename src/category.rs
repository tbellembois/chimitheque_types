use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
    #[serde(default)]
    pub match_exact_search: bool,
    pub category_id: Option<u64>,
    pub category_label: String,
}

impl Category {
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        self.category_label = clean(&self.category_label, Transform::None);
        if self.category_label.is_empty() {
            return Err(ValidationError::Empty {
                field: "category_label",
            });
        }
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
