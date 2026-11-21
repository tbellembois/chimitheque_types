use serde::{Deserialize, Serialize};

/// A class of compounds linked to a product, with its label.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productclassesofcompounds {
    /// Id of the product.
    pub productclassesofcompounds_product_id: u64,
    /// Id of the class of compound.
    pub productclassesofcompounds_class_of_compound_id: u64,
    /// Label of the class of compound.
    pub productclassesofcompounds_class_of_compound_label: String,
}

#[cfg(test)]
#[path = "productclassesofcompounds_tests.rs"]
mod productclassesofcompounds_tests;
