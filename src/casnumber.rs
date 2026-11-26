use chimitheque_utils::{
    casnumber::is_cas_number,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;
use crate::validation::{check_format, clean_optional_text, is_cmr_classification};

/// A CAS registry number identifying a chemical substance, e.g. `7732-18-5`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CasNumber {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new cas number.
    pub cas_number_id: Option<u64>,
    /// CAS number, e.g. `7732-18-5`.
    pub cas_number_label: String,
    /// Space-separated CMR codes (C1A, C1B, C2, M1A, M1B, M2, R1A, R1B, R2, LACT), if the substance is CMR.
    pub cas_number_cmr: Option<String>,
}

impl CasNumber {
    /// Cleans the CAS number and its CMR codes, and checks that the CAS number is
    /// valid and the CMR codes are known. An empty CMR value becomes `None`.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty`, `ValidationError::InvalidCasNumber` or
    /// `ValidationError::InvalidFormat`.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        self.cas_number_label = clean(&self.cas_number_label, Transform::None);
        if self.cas_number_label.is_empty() {
            return Err(ValidationError::Empty {
                field: "cas_number_label",
            });
        }
        clean_optional_text(&mut self.cas_number_cmr);
        check_format(
            self.cas_number_cmr.as_deref(),
            "cas_number_cmr",
            is_cmr_classification,
            "space-separated CMR codes (C1A, C1B, C2, M1A, M1B, M2, R1A, R1B, R2, LACT)",
        )?;
        is_cas_number(&self.cas_number_label).map_err(ValidationError::InvalidCasNumber)
    }
}

impl_searchable!(CasNumber, table: "cas_number", id: cas_number_id, text: cas_number_label);

impl fmt::Display for CasNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cas_number_label)
    }
}

#[cfg(test)]
#[path = "casnumber_tests.rs"]
mod casnumber_tests;
