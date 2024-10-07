use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrecautionaryStatement {
    pub match_exact_search: bool,
    pub precautionary_statement_id: u64,
    pub precautionary_statement_label: String,
    pub precautionary_statement_reference: String,
}
