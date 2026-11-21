use chimitheque_utils::{
    formula::to_empirical_formula,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::ValidationError;
use crate::searchable::impl_searchable;

/// A linear (condensed structural) formula, e.g. `CH3CH2OH`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LinearFormula {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new linear formula.
    pub linear_formula_id: Option<u64>,
    /// Linear formula, e.g. `CH3CH2OH`.
    pub linear_formula_label: String,
}

impl LinearFormula {
    /// Cleans the formula, ensures it is not empty and can be converted into an
    /// empirical formula.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` or `ValidationError::InvalidFormula`.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        let cleaned_empirical_formula = clean(&self.linear_formula_label, Transform::None);
        if cleaned_empirical_formula.is_empty() {
            return Err(ValidationError::Empty {
                field: "linear_formula_label",
            });
        }

        let _ = to_empirical_formula(cleaned_empirical_formula.as_str()).map_err(|source| {
            ValidationError::InvalidFormula {
                field: "linear_formula_label",
                source,
            }
        })?;

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
