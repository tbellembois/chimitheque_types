use crate::producer::Producer;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProducerRef {
    #[serde(default)]
    pub match_exact_search: bool,
    pub producer_ref_id: Option<u64>,
    pub producer_ref_label: String,

    pub producer: Producer,
}

impl ProducerRef {
    /// Cleans the reference label and ensures it is not empty. A new producer
    /// (without an id) is validated too; an existing one is left unchanged.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the label or a new producer label is
    /// empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        require_text(
            &mut self.producer_ref_label,
            "producer_ref_label",
            Transform::None,
        )?;
        if self.producer.producer_id.is_none() {
            self.producer.sanitize_and_validate()?;
        }
        Ok(())
    }
}

impl fmt::Display for ProducerRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Producer: {})",
            self.producer_ref_label, self.producer.producer_label
        )
    }
}

#[cfg(test)]
#[path = "producerref_tests.rs"]
mod producerref_tests;
