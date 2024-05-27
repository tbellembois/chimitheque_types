use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Producer {
    pub match_exact_search: bool,
    pub producer_id: u64,
    pub producer_label: String,
}
