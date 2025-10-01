use chimitheque_traits::searchable::Searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HazardStatement {
    pub match_exact_search: bool,
    pub hazard_statement_id: Option<u64>,
    pub hazard_statement_label: String,
    pub hazard_statement_reference: String,
    pub hazard_statement_cmr: Option<String>,
}

impl Searchable for HazardStatement {
    fn create(&self) -> Self {
        HazardStatement {
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
        String::from("hazard_statement")
    }

    fn get_id_field_name(&self) -> String {
        String::from("hazard_statement_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.hazard_statement_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("hazard_statement_reference")
    }

    fn set_text_field(&mut self, text: &str) {
        self.hazard_statement_reference = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.hazard_statement_id
    }

    fn get_text(&self) -> String {
        self.hazard_statement_reference.clone()
    }
}
