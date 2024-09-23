use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productclassofcompound {
    pub productclassofcompound_product_id: u64,
    pub productclassofcompound_classofcompound_id: u64,
    pub productclassofcompound_classofcompound_label: String,
}
