use crate::error::ValidationError;
use crate::searchable::impl_searchable;
use chimitheque_defines::PRECAUTIONARY_REFERENCE_RE;
use chimitheque_utils::string::{Transform, clean_required_text};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A GHS precautionary statement, e.g. `P280` "Wear protective gloves".
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrecautionaryStatement {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new precautionary statement.
    pub precautionary_statement_id: Option<u64>,
    /// Statement text.
    pub precautionary_statement_label: String,
    /// GHS reference, e.g. `P280` or `P301+P317`.
    pub precautionary_statement_reference: String,
}

impl PrecautionaryStatement {
    /// Cleans the label and reference and ensures they are not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the label or reference is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label and the reference are required.
        self.precautionary_statement_label =
            clean_required_text(&self.precautionary_statement_label, Transform::None).ok_or(
                ValidationError::Empty {
                    field: "precautionary_statement_label",
                },
            )?;
        self.precautionary_statement_reference =
            clean_required_text(&self.precautionary_statement_reference, Transform::None).ok_or(
                ValidationError::Empty {
                    field: "precautionary_statement_reference",
                },
            )?;
        // The reference must be a GHS precautionary reference.
        if !is_precautionary_reference(&self.precautionary_statement_reference) {
            return Err(ValidationError::InvalidFormat {
                field: "precautionary_statement_reference",
                expected: "a GHS precautionary reference like P280 or P301+P317",
            });
        }
        Ok(())
    }
}

/// Returns true for a GHS precautionary statement reference, e.g. `P280` or
/// `P301+P317`.
fn is_precautionary_reference(reference: &str) -> bool {
    PRECAUTIONARY_REFERENCE_RE.is_match(reference)
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
