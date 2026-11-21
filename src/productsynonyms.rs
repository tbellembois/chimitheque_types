use serde::{Deserialize, Serialize};

/// A synonym (name) linked to a product, with its label.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsynonyms {
    /// Id of the product.
    pub productsynonyms_product_id: u64,
    /// Id of the name.
    pub productsynonyms_name_id: u64,
    /// Label of the name.
    pub productsynonyms_name_label: String,
}

#[cfg(test)]
#[path = "productsynonyms_tests.rs"]
mod productsynonyms_tests;
