use chimitheque_utils::{
    cenumber::is_ce_number,
    string::{Transform, clean_required_text},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A European Community (EC) number identifying a substance, e.g. `231-791-2`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CeNumber {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new ce number.
    pub ce_number_id: Option<u64>,
    /// EC number, e.g. `231-791-2`.
    pub ce_number_label: String,
}

impl CeNumber {
    /// Cleans the EC number and checks that it is valid.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` or `ValidationError::InvalidCeNumber`.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        self.ce_number_label = clean_required_text(&self.ce_number_label, Transform::None).ok_or(
            ValidationError::Empty {
                field: "ce_number_label",
            },
        )?;
        // The label must be a valid EC number.
        is_ce_number(&self.ce_number_label).map_err(ValidationError::InvalidCeNumber)
    }
}

impl_searchable!(CeNumber, table: "ce_number", id: ce_number_id, text: ce_number_label);

impl fmt::Display for CeNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ce_number_label)
    }
}

#[cfg(test)]
#[path = "cenumber_tests.rs"]
mod cenumber_tests;
