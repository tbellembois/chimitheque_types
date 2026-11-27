use crate::error::ValidationError;
use crate::supplier::Supplier;
use chimitheque_utils::string::{Transform, clean_required_text};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A supplier's catalogue reference for a product.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SupplierRef {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new supplier ref.
    pub supplier_ref_id: Option<u64>,
    /// Reference label.
    pub supplier_ref_label: String,

    /// Supplier the reference belongs to.
    pub supplier: Supplier,
}

impl SupplierRef {
    /// Cleans the reference label and ensures it is not empty. A new supplier
    /// (without an id) is validated too; an existing one is left unchanged.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the label or a new supplier label is
    /// empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        self.supplier_ref_label = clean_required_text(&self.supplier_ref_label, Transform::None)
            .ok_or(ValidationError::Empty {
                field: "supplier_ref_label",
            })?;
        // A new supplier (without id) is validated too; an existing one is
        // already in the database.
        if self.supplier.supplier_id.is_none() {
            self.supplier.sanitize_and_validate()?;
        }
        Ok(())
    }
}

impl fmt::Display for SupplierRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (supplier: {})",
            self.supplier_ref_label, self.supplier.supplier_label
        )
    }
}

#[cfg(test)]
#[path = "supplierref_tests.rs"]
mod supplierref_tests;
