use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Name {
    #[serde(default)]
    pub match_exact_search: bool,
    pub name_id: Option<u64>,
    pub name_label: String,
}

impl Name {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.name_label = clean(&self.name_label, Transform::Uppercase);
        if self.name_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        Ok(())
    }
}

impl_searchable!(Name, table: "name", id: name_id, text: name_label);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_label)
    }
}

#[cfg(test)]
#[path = "name_tests.rs"]
mod name_tests;
