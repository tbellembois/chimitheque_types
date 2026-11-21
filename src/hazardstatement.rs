use crate::searchable::impl_searchable;
use crate::validation::clean_optional_text;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
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

impl HazardStatement {
    /// Cleans the label, reference and CMR classification, and ensures the
    /// label and reference are not empty. An empty CMR classification becomes
    /// `None`.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the label or reference is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        require_text(
            &mut self.hazard_statement_label,
            "hazard_statement_label",
            Transform::None,
        )?;
        require_text(
            &mut self.hazard_statement_reference,
            "hazard_statement_reference",
            Transform::None,
        )?;
        clean_optional_text(&mut self.hazard_statement_cmr);
        Ok(())
    }
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
