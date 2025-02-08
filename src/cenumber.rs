use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CeNumber {
    pub match_exact_search: bool,
    pub ce_number_id: u64,
    pub ce_number_label: String,
}

impl Searchable for CeNumber {
    fn create(&self) -> Self {
        CeNumber {
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
        String::from("ce_number")
    }

    fn get_id_field_name(&self) -> String {
        String::from("ce_number_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.ce_number_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("ce_number_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.ce_number_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.ce_number_id
    }

    fn get_text(&self) -> String {
        self.ce_number_label.clone()
    }
}
