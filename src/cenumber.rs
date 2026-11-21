use chimitheque_utils::{
    cenumber::is_ce_number,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CeNumber {
    #[serde(default)]
    pub match_exact_search: bool,
    pub ce_number_id: Option<u64>,
    pub ce_number_label: String,
}

impl CeNumber {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.ce_number_label = clean(&self.ce_number_label, Transform::None);
        if self.ce_number_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        is_ce_number(&self.ce_number_label)
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
