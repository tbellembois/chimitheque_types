use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producthazardstatements {
    pub producthazardstatements_product_id: u64,
    pub producthazardstatements_hazard_statement_id: u64,
    pub producthazardstatements_hazard_statement_label: String,
    pub producthazardstatements_hazard_statement_reference: String,
    pub producthazardstatements_hazard_statement_cmr: Option<String>,
}
