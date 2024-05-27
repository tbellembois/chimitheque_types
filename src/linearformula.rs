use chimitheque_traits::searchable::Searchable;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Linearformula {
    pub match_exact_search: bool,
    pub linearformula_id: u64,
    pub linearformula_label: String,
}

impl Searchable for Linearformula {
    fn new(&self) -> Self {
        Linearformula {
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
        String::from("linearformula")
    }

    fn get_id_field_name(&self) -> String {
        String::from("linearformula_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.linearformula_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("linearformula_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.linearformula_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.linearformula_id
    }

    fn get_text(&self) -> String {
        self.linearformula_label.clone()
    }
}
