use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LinearFormula {
    pub match_exact_search: bool,
    pub linear_formula_id: u64,
    pub linear_formula_label: String,
}

impl Searchable for LinearFormula {
    fn new(&self) -> Self {
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
        self.linear_formula_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("linear_formula_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.linear_formula_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.linear_formula_id
    }

    fn get_text(&self) -> String {
        self.linear_formula_label.clone()
    }
}
