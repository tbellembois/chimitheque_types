use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
    pub match_exact_search: bool,
    pub category_id: Option<u64>,
    pub category_label: String,
}

impl Category {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.category_label = clean(&self.category_label, Transform::None);
        if self.category_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        Ok(())
    }
}

impl Searchable for Category {
    fn create(&self) -> Self {
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
        self.category_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("category_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.category_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.category_id
    }

    fn get_text(&self) -> String {
        self.category_label.clone()
    }
}

#[cfg(test)]
#[path = "category_tests.rs"]
mod category_tests;
