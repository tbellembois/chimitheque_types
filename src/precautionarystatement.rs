use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Precautionarystatement {
    pub match_exact_search: bool,
    pub precautionarystatement_id: u64,
    pub precautionarystatement_label: String,
    pub precautionarystatement_reference: String,
}
