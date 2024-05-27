use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Supplier {
    pub match_exact_search: bool,
    pub supplier_id: u64,
    pub supplier_label: String,
}
