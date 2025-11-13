use std::error::Error;

use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::formula::sort_empirical_formula;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmpiricalFormula {
    pub match_exact_search: bool,
    pub empirical_formula_id: Option<u64>,
    pub empirical_formula_label: String,
}

impl EmpiricalFormula {
    pub fn is_valid(&self) -> Result<bool, Box<dyn Error>> {
        match sort_empirical_formula(&self.empirical_formula_label) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
impl Searchable for EmpiricalFormula {
    fn create(&self) -> Self {
        EmpiricalFormula {
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
        String::from("empirical_formula")
    }

    fn get_id_field_name(&self) -> String {
        String::from("empirical_formula_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.empirical_formula_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("empirical_formula_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.empirical_formula_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.empirical_formula_id
    }

    fn get_text(&self) -> String {
        self.empirical_formula_label.clone()
    }
}
