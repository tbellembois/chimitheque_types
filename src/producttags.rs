use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producttags {
    pub producttags_product_id: u64,
    pub producttags_tag_id: u64,
    pub producttags_tag_label: String,
}
