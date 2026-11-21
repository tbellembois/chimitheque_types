use serde::{Deserialize, Serialize};

/// A hazard symbol linked to a product, with its label.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsymbols {
    /// Id of the product.
    pub productsymbols_product_id: u64,
    /// Id of the symbol.
    pub productsymbols_symbol_id: u64,
    /// Label of the symbol.
    pub productsymbols_symbol_label: String,
}

#[cfg(test)]
#[path = "productsymbols_tests.rs"]
mod productsymbols_tests;
