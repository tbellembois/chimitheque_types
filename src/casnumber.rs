use chimitheque_utils::{
    casnumber::is_cas_number,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CasNumber {
    #[serde(default)]
    pub match_exact_search: bool,
    pub cas_number_id: Option<u64>,
    pub cas_number_label: String,
    pub cas_number_cmr: Option<String>,
}

impl CasNumber {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.cas_number_label = clean(&self.cas_number_label, Transform::None);
        if self.cas_number_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        is_cas_number(&self.cas_number_label)
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
