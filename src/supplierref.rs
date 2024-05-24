use crate::supplier::Supplier;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Supplierref {
    pub match_exact_search: bool,
    pub supplierref_id: u64,
    pub supplierref_label: String,

    pub supplier: Supplier,
}
