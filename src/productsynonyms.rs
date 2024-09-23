use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsynonyms {
    pub productsynonyms_product_id: u64,
    pub productsynonyms_name_id: u64,
    pub productsynonyms_name_label: String,
}
