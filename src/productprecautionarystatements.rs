use serde::{Deserialize, Serialize};

/// A precautionary statement linked to a product, with its details.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productprecautionarystatements {
    /// Id of the product.
    pub productprecautionarystatements_product_id: u64,
    /// Id of the precautionary statement.
    pub productprecautionarystatements_precautionary_statement_id: u64,
    /// Label of the precautionary statement.
    pub productprecautionarystatements_precautionary_statement_label: String,
    /// Reference of the precautionary statement.
    pub productprecautionarystatements_precautionary_statement_reference: String,
}

#[cfg(test)]
#[path = "productprecautionarystatements_tests.rs"]
mod productprecautionarystatements_tests;
