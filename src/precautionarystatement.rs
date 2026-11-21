use crate::searchable::impl_searchable;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrecautionaryStatement {
    #[serde(default)]
    pub match_exact_search: bool,
    pub precautionary_statement_id: Option<u64>,
    pub precautionary_statement_label: String,
    pub precautionary_statement_reference: String,
}

impl_searchable!(PrecautionaryStatement, table: "precautionary_statement", id: precautionary_statement_id, text: precautionary_statement_reference);

impl fmt::Display for PrecautionaryStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.precautionary_statement_label, self.precautionary_statement_reference
        )
    }
}

#[cfg(test)]
#[path = "precautionarystatement_tests.rs"]
mod precautionarystatement_tests;
