use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productsupplierrefs {
    pub productsupplierrefs_product_id: u64,
    pub productsupplierrefs_supplierref_id: u64,
    pub productsupplierrefs_supplierref_label: String,
    pub productsupplierrefs_supplier_id: u64,
    pub productsupplierrefs_supplier_label: String,
}
