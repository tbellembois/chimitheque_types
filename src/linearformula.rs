use chimitheque_utils::{
    formula::to_empirical_formula,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LinearFormula {
    #[serde(default)]
    pub match_exact_search: bool,
    pub linear_formula_id: Option<u64>,
    pub linear_formula_label: String,
}

impl LinearFormula {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let cleaned_empirical_formula = clean(&self.linear_formula_label, Transform::None);
        if cleaned_empirical_formula.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }

        let _ = to_empirical_formula(cleaned_empirical_formula.as_str())?;

        self.linear_formula_label = cleaned_empirical_formula;
        Ok(())
    }
}

impl_searchable!(LinearFormula, table: "linear_formula", id: linear_formula_id, text: linear_formula_label);

impl fmt::Display for LinearFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.linear_formula_label)
    }
}

#[cfg(test)]
#[path = "linearformula_tests.rs"]
mod linearformula_tests;
