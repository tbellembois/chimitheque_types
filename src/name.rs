use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Name {
    pub match_exact_search: bool,
    pub name_id: u64,
    pub name_label: String,
}

impl Searchable for Name {
    fn new(&self) -> Self {
        Name {
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
        String::from("name")
    }

    fn get_id_field_name(&self) -> String {
        String::from("name_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.name_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("name_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.name_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.name_id
    }

    fn get_text(&self) -> String {
        self.name_label.clone()
    }
}
