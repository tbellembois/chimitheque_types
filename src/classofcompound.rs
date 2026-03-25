use chimitheque_traits::searchable::Searchable;
use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

use crate::error::ParseError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassOfCompound {
    pub match_exact_search: bool,
    pub class_of_compound_id: Option<u64>,
    pub class_of_compound_label: String,
}

impl ClassOfCompound {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.class_of_compound_label = clean(&self.class_of_compound_label, Transform::None);
        if self.class_of_compound_label.is_empty() {
            return Err(Box::new(ParseError::EmptyInput));
        }
        Ok(())
    }
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
        self.class_of_compound_id = Some(id);
    }

    fn get_text_field_name(&self) -> String {
        String::from("class_of_compound_label")
    }

    fn set_text_field(&mut self, text: &str) {
        self.class_of_compound_label = text.to_string();
    }

    fn get_id(&self) -> Option<u64> {
        self.class_of_compound_id
    }

    fn get_text(&self) -> String {
        self.class_of_compound_label.clone()
    }
}

#[cfg(test)]
#[path = "classofcompound_tests.rs"]
mod classofcompound_tests;
