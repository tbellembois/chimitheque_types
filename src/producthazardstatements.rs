use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producthazardstatements {
    pub producthazardstatements_product_id: u64,
    pub producthazardstatements_hazardstatement_id: u64,
    pub producthazardstatements_hazardstatement_label: String,
    pub producthazardstatements_hazardstatement_reference: String,
    pub producthazardstatements_hazardstatement_cmr: Option<String>,
}
