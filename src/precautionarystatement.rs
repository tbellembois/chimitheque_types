use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrecautionaryStatement {
    pub match_exact_search: bool,
    pub precautionary_statement_id: u64,
    pub precautionary_statement_label: String,
    pub precautionary_statement_reference: String,
}

impl Searchable for PrecautionaryStatement {
    fn new(&self) -> Self {
        PrecautionaryStatement {
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
        String::from("precautionary_statement")
    }

    fn get_id_field_name(&self) -> String {
        String::from("precautionary_statement_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.precautionary_statement_id = id;
    }

    fn get_text_field_name(&self) -> String {
        String::from("precautionary_statement_reference")
    }

    fn set_text_field(&mut self, text: &str) {
        self.precautionary_statement_reference = text.to_string();
    }

    fn get_id(&self) -> u64 {
        self.precautionary_statement_id
    }

    fn get_text(&self) -> String {
        self.precautionary_statement_reference.clone()
    }
}
