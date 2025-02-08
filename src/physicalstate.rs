use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PhysicalState {
    pub match_exact_search: bool,
    pub physical_state_id: u64,
    pub physical_state_label: String,
}

impl Searchable for PhysicalState {
    fn create(&self) -> Self {
        PhysicalState {
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
        String::from("physical_state")
    }

    fn get_id_field_name(&self) -> String {
        String::from("physical_state_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.physical_state_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("physical_state_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.physical_state_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.physical_state_id
    }

    fn get_text(&self) -> String {
        self.physical_state_label.clone()
    }
}
