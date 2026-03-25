use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    pub match_exact_search: bool,
    pub tag_id: Option<u64>,
    pub tag_label: String,
}

impl Tag {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.tag_label = clean(&self.tag_label, Transform::None);
        if self.tag_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        Ok(())
    }
}

impl Searchable for Tag {
    fn create(&self) -> Self {
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
        self.tag_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("tag_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.tag_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.tag_id
    }

    fn get_text(&self) -> String {
        self.tag_label.clone()
    }
}

#[cfg(test)]
#[path = "tag_tests.rs"]
mod tag_tests;
