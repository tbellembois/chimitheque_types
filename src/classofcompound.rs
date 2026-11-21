use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassOfCompound {
    #[serde(default)]
    pub match_exact_search: bool,
    pub class_of_compound_id: Option<u64>,
    pub class_of_compound_label: String,
}

impl ClassOfCompound {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.class_of_compound_label = clean(&self.class_of_compound_label, Transform::None);
        if self.class_of_compound_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        Ok(())
    }
}

impl_searchable!(ClassOfCompound, table: "class_of_compound", id: class_of_compound_id, text: class_of_compound_label);

impl fmt::Display for ClassOfCompound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_of_compound_label)
    }
}

#[cfg(test)]
#[path = "classofcompound_tests.rs"]
mod classofcompound_tests;
