use crate::searchable::impl_searchable;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};

/// A GHS signal word, `danger` or `warning`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignalWord {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new signal word.
    pub signal_word_id: Option<u64>,
    /// Signal word.
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
