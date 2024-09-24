use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productprecautionarystatements {
    pub productprecautionarystatements_product_id: u64,
    pub productprecautionarystatements_precautionarystatement_id: u64,
    pub productprecautionarystatements_precautionarystatement_label: String,
    pub productprecautionarystatements_precautionarystatement_reference: String,
}
