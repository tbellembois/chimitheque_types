use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Productprecautionarystatements {
    pub productprecautionarystatements_product_id: u64,
    pub productprecautionarystatements_precautionary_statement_id: u64,
    pub productprecautionarystatements_precautionary_statement_label: String,
    pub productprecautionarystatements_precautionary_statement_reference: String,
}
