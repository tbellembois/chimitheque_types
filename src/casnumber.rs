use std::error::Error;

use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::casnumber::is_cas_number;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CasNumber {
    pub match_exact_search: bool,
    pub cas_number_id: Option<u64>,
    pub cas_number_label: String,
}

impl CasNumber {
    pub fn is_valid(&self) -> Result<bool, Box<dyn Error>> {
        is_cas_number(&self.cas_number_label)
    }
}

impl Searchable for CasNumber {
    fn create(&self) -> Self {
        CasNumber {
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
        String::from("cas_number")
    }

    fn get_id_field_name(&self) -> String {
        String::from("cas_number_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.cas_number_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("cas_number_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.cas_number_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.cas_number_id
    }

    fn get_text(&self) -> String {
        self.cas_number_label.clone()
    }
}
