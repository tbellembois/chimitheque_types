use crate::searchable::impl_searchable;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
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

impl PrecautionaryStatement {
    /// Cleans the label and reference and ensures they are not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the label or reference is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        require_text(
            &mut self.precautionary_statement_label,
            "precautionary_statement_label",
            Transform::None,
        )?;
        require_text(
            &mut self.precautionary_statement_reference,
            "precautionary_statement_reference",
            Transform::None,
        )?;
        Ok(())
    }
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
