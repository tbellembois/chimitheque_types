use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsymbols {
    pub productsymbols_product_id: u64,
    pub productsymbols_symbol_id: u64,
    pub productsymbols_symbol_label: String,
}
