use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producer {
    #[serde(default)]
    pub match_exact_search: bool,
    pub producer_id: Option<u64>,
    pub producer_label: String,
}

impl Producer {
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
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
