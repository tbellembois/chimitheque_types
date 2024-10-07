use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HazardStatement {
    pub match_exact_search: bool,
    pub hazard_statement_id: u64,
    pub hazard_statement_label: String,
    pub hazard_statement_reference: String,
    pub hazard_statement_cmr: Option<String>,
}
