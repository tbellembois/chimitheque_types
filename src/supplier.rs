use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Supplier {
    #[serde(default)]
    pub match_exact_search: bool,
    pub supplier_id: Option<u64>,
    pub supplier_label: String,
}

impl Supplier {
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
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
