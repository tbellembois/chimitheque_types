use chimitheque_utils::{
    formula::to_empirical_formula,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ParseError;
use crate::searchable::impl_searchable;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmpiricalFormula {
    #[serde(default)]
    pub match_exact_search: bool,
    pub empirical_formula_id: Option<u64>,
    pub empirical_formula_label: String,
}

impl EmpiricalFormula {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sorted_and_cleaned_empirical_formula =
            to_empirical_formula(&clean(&self.empirical_formula_label, Transform::None))?;
        if sorted_and_cleaned_empirical_formula.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        self.empirical_formula_label = sorted_and_cleaned_empirical_formula;
        Ok(())
    }
}

impl_searchable!(EmpiricalFormula, table: "empirical_formula", id: empirical_formula_id, text: empirical_formula_label);

impl fmt::Display for EmpiricalFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.empirical_formula_label)
    }
}

#[cfg(test)]
#[path = "empiricalformula_tests.rs"]
mod empiricalformula_tests;
