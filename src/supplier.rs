use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::string::{clean, Transform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Supplier {
    #[serde(default)]
    pub match_exact_search: bool,
    pub supplier_id: Option<u64>,
    pub supplier_label: String,
}

impl Supplier {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.supplier_label = clean(&self.supplier_label, Transform::None);
        Ok(())
    }
}

impl Searchable for Supplier {
    fn create(&self) -> Self {
        Supplier {
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
        String::from("supplier")
    }

    fn get_id_field_name(&self) -> String {
        String::from("supplier_id")
    }

    fn set_id_field(&mut self, id: u64) {
        self.supplier_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("supplier_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.supplier_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.supplier_id
    }

    fn get_text(&self) -> String {
        self.supplier_label.clone()
    }
}
