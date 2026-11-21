use crate::searchable::impl_searchable;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HazardStatement {
    #[serde(default)]
    pub match_exact_search: bool,
    pub hazard_statement_id: Option<u64>,
    pub hazard_statement_label: String,
    pub hazard_statement_reference: String,
    pub hazard_statement_cmr: Option<String>,
}

impl_searchable!(HazardStatement, table: "hazard_statement", id: hazard_statement_id, text: hazard_statement_reference);

impl fmt::Display for HazardStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.hazard_statement_label, self.hazard_statement_reference
        )
    }
}

#[cfg(test)]
#[path = "hazardstatement_tests.rs"]
mod hazardstatement_tests;
