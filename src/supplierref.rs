use crate::supplier::Supplier;
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

impl fmt::Display for SupplierRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (supplier: {})",
            self.supplier_ref_label, self.supplier.supplier_label
        )
    }
}
