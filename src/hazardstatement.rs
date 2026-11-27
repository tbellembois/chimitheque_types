use crate::searchable::impl_searchable;
use crate::validation::{clean_optional_text, is_cmr_classification};
use crate::{error::ValidationError, validation::clean_required_text};
use chimitheque_defines::HAZARD_REFERENCE_RE;
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A GHS hazard statement, e.g. `H350` "May cause cancer".
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HazardStatement {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new hazard statement.
    pub hazard_statement_id: Option<u64>,
    /// Statement text.
    pub hazard_statement_label: String,
    /// GHS reference, e.g. `H350` or `H300+H310`.
    pub hazard_statement_reference: String,
    /// CMR classification codes associated with the statement, if any.
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
        // The label and the reference are required.
        clean_required_text(
            &mut self.hazard_statement_label,
            "hazard_statement_label",
            Transform::None,
        )?;
        clean_required_text(
            &mut self.hazard_statement_reference,
            "hazard_statement_reference",
            Transform::None,
        )?;
        // The CMR classification is optional; an empty one becomes `None`.
        clean_optional_text(&mut self.hazard_statement_cmr);
        // The reference must be a GHS hazard reference.
        if !is_hazard_reference(&self.hazard_statement_reference) {
            return Err(ValidationError::InvalidFormat {
                field: "hazard_statement_reference",
                expected: "a GHS hazard reference like H350 or H300+H310",
            });
        }
        // A set CMR classification must contain known CMR codes.
        if let Some(cmr) = self.hazard_statement_cmr.as_deref()
            && !is_cmr_classification(cmr)
        {
            return Err(ValidationError::InvalidFormat {
                field: "hazard_statement_cmr",
                expected: "space-separated CMR codes (C1A, C1B, C2, M1A, M1B, M2, R1A, R1B, R2, LACT)",
            });
        }
        Ok(())
    }
}

/// Returns true for a GHS hazard statement reference, e.g. `H350` or `H300+H310`.
fn is_hazard_reference(reference: &str) -> bool {
    HAZARD_REFERENCE_RE.is_match(reference)
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
