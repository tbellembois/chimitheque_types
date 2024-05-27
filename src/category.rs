use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Category {
    pub match_exact_search: bool,
    pub category_id: u64,
    pub category_label: String,
}

impl Searchable for Category {
    fn new(&self) -> Self {
        Category {
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
        String::from("category")
    }

    fn get_id_field_name(&self) -> String {
        String::from("category_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.category_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("category_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.category_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.category_id
    }

    fn get_text(&self) -> String {
        self.category_label.clone()
    }
}
