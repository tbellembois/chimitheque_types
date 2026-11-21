use serde::{Deserialize, Serialize};

/// A supplier reference linked to a product, with its supplier.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsupplierrefs {
    /// Id of the product.
    pub productsupplierrefs_product_id: u64,
    /// Id of the supplier ref.
    pub productsupplierrefs_supplier_ref_id: u64,
    /// Label of the supplier ref.
    pub productsupplierrefs_supplier_ref_label: String,
    /// Id of the supplier.
    pub productsupplierrefs_supplier_id: u64,
    /// Label of the supplier.
    pub productsupplierrefs_supplier_label: String,
}

#[cfg(test)]
#[path = "productsupplierrefs_tests.rs"]
mod productsupplierrefs_tests;
