use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassOfCompound {
    pub match_exact_search: bool,
    pub class_of_compound_id: u64,
    pub class_of_compound_label: String,
}

impl Searchable for ClassOfCompound {
    fn create(&self) -> Self {
        ClassOfCompound {
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
        String::from("class_of_compound")
    }

    fn get_id_field_name(&self) -> String {
        String::from("class_of_compound_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.class_of_compound_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("class_of_compound_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.class_of_compound_label = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.class_of_compound_id
    }

    fn get_text(&self) -> String {
        self.class_of_compound_label.clone()
    }
}
