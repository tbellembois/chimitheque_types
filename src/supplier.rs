use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Supplier {
    #[serde(default)]
    pub match_exact_search: bool,
    pub supplier_id: Option<u64>,
    pub supplier_label: String,
}
