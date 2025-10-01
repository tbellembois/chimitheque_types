use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Symbol {
    pub match_exact_search: bool,
    pub symbol_id: Option<u64>,
    pub symbol_label: String,
}

impl Searchable for Symbol {
    fn create(&self) -> Self {
        Symbol {
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
        String::from("symbol")
    }

    fn get_id_field_name(&self) -> String {
        String::from("symbol_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.symbol_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("symbol_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.symbol_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.symbol_id
    }

    fn get_text(&self) -> String {
        self.symbol_label.clone()
    }
}
