use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A product producer (manufacturer).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producer {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new producer.
    pub producer_id: Option<u64>,
    /// Producer name.
    pub producer_label: String,
}

impl Producer {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        self.producer_label = clean(&self.producer_label, Transform::None);
        if self.producer_label.is_empty() {
            return Err(ValidationError::Empty {
                field: "producer_label",
            });
        }
        Ok(())
    }
}

impl_searchable!(Producer, table: "producer", id: producer_id, text: producer_label);

#[cfg(test)]
#[path = "producer_tests.rs"]
mod producer_tests;
