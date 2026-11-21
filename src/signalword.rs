use crate::searchable::impl_searchable;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignalWord {
    #[serde(default)]
    pub match_exact_search: bool,
    pub signal_word_id: Option<u64>,
    pub signal_word_label: String,
}

impl SignalWord {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        require_text(
            &mut self.signal_word_label,
            "signal_word_label",
            Transform::None,
        )?;
        Ok(())
    }
}

impl_searchable!(SignalWord, table: "signal_word", id: signal_word_id, text: signal_word_label);

#[cfg(test)]
#[path = "signalword_tests.rs"]
mod signalword_tests;
