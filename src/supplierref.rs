use crate::supplier::Supplier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SupplierRef {
    pub match_exact_search: bool,
    pub supplier_ref_id: Option<u64>,
    pub supplier_ref_label: String,

    pub supplier: Supplier,
}
