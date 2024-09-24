use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Hazardstatement {
    pub match_exact_search: bool,
    pub hazardstatement_id: u64,
    pub hazardstatement_label: String,
    pub hazardstatement_reference: String,
    pub hazardstatement_cmr: Option<String>,
}
