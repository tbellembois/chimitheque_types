use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::{
    formula::to_empirical_formula,
    string::{Transform, clean},
};
use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LinearFormula {
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

impl Searchable for LinearFormula {
    fn create(&self) -> Self {
        LinearFormula {
            ..Default::default()
        }
    }

    fn set_exact_search(&mut self, match_exact_search: bool) {
        self.match_exact_search = match_exact_search;
    }

    fn get_exact_search(&self) -> bool {
        self.match_exact_search
    }

    fn get_table_name(&self) -> String {
        String::from("linear_formula")
    }

    fn get_id_field_name(&self) -> String {
        String::from("linear_formula_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.linear_formula_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("linear_formula_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.linear_formula_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.linear_formula_id
    }

    fn get_text(&self) -> String {
        self.linear_formula_label.clone()
    }
}

#[cfg(test)]
#[path = "linearformula_tests.rs"]
mod linearformula_tests;
