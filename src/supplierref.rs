use crate::supplier::Supplier;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SupplierRef {
    #[serde(default)]
    pub match_exact_search: bool,
    pub supplier_ref_id: Option<u64>,
    pub supplier_ref_label: String,

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
        require_text(
            &mut self.supplier_ref_label,
            "supplier_ref_label",
            Transform::None,
        )?;
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
