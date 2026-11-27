use chimitheque_utils::{
    formula::to_empirical_formula,
    string::{Transform, clean_required_text},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// An empirical formula, stored sorted (C, H, then alphabetical), e.g. `C2H6O`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmpiricalFormula {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new empirical formula.
    pub empirical_formula_id: Option<u64>,
    /// Empirical formula, e.g. `C2H6O`.
    pub empirical_formula_label: String,
}

impl EmpiricalFormula {
    /// Cleans and sorts the formula (C, H, then alphabetical) and ensures it is
    /// not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the formula is empty or has no atom,
    /// or `ValidationError::InvalidFormula` if it can not be parsed.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        let cleaned_empirical_formula =
            clean_required_text(&self.empirical_formula_label, Transform::None).ok_or(
                ValidationError::Empty {
                    field: "empirical_formula_label",
                },
            )?;
        // The label must be a valid formula; it is stored as its sorted empirical formula.
        let sorted_and_cleaned_empirical_formula = to_empirical_formula(&cleaned_empirical_formula)
            .map_err(|source| ValidationError::InvalidFormula {
                field: "empirical_formula_label",
                source,
            })?;
        // A formula without any atom, e.g. `()` or `x`, gives an empty formula.
        if sorted_and_cleaned_empirical_formula.is_empty() {
            return Err(ValidationError::Empty {
                field: "empirical_formula_label",
            });
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
