use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::string::{clean, Transform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producer {
    #[serde(default)]
    pub match_exact_search: bool,
    pub producer_id: Option<u64>,
    pub producer_label: String,
}

impl Producer {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.producer_label = clean(&self.producer_label, Transform::None);
        Ok(())
    }
}

impl Searchable for Producer {
    fn create(&self) -> Self {
        Producer {
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
        String::from("producer")
    }

    fn get_id_field_name(&self) -> String {
        String::from("producer_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.producer_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("producer_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.producer_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.producer_id
    }

    fn get_text(&self) -> String {
        self.producer_label.clone()
    }
}
