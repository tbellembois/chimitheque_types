use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsupplierrefs {
    pub productsupplierrefs_product_id: u64,
    pub productsupplierrefs_supplier_ref_id: u64,
    pub productsupplierrefs_supplier_ref_label: String,
    pub productsupplierrefs_supplier_id: u64,
    pub productsupplierrefs_supplier_label: String,
}
