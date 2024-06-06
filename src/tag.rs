use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    pub match_exact_search: bool,
    pub tag_id: u64,
    pub tag_label: String,
}

impl Searchable for Tag {
    fn new(&self) -> Self {
        Tag {
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
        String::from("tag")
    }

    fn get_id_field_name(&self) -> String {
        String::from("tag_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.tag_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("tag_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.tag_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.tag_id
    }

    fn get_text(&self) -> String {
        self.tag_label.clone()
    }
}
