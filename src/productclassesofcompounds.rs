use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productclassesofcompounds {
    pub productclassesofcompounds_product_id: u64,
    pub productclassesofcompounds_class_of_compound_id: u64,
    pub productclassesofcompounds_class_of_compound_label: String,
}
