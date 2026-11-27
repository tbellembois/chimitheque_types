use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A product supplier (vendor).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Supplier {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new supplier.
    pub supplier_id: Option<u64>,
    /// Supplier name.
    pub supplier_label: String,
}

impl Supplier {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        self.supplier_label = clean(&self.supplier_label, Transform::None);
        if self.supplier_label.is_empty() {
            return Err(ValidationError::Empty {
                field: "supplier_label",
            });
        }
        Ok(())
    }
}

impl_searchable!(Supplier, table: "supplier", id: supplier_id, text: supplier_label);

#[cfg(test)]
#[path = "supplier_tests.rs"]
mod supplier_tests;
