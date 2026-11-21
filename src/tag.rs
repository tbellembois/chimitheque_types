use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    #[serde(default)]
    pub match_exact_search: bool,
    pub tag_id: Option<u64>,
    pub tag_label: String,
}

impl Tag {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.tag_label = clean(&self.tag_label, Transform::None);
        if self.tag_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        Ok(())
    }
}

impl_searchable!(Tag, table: "tag", id: tag_id, text: tag_label);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag_label)
    }
}

#[cfg(test)]
#[path = "tag_tests.rs"]
mod tag_tests;
