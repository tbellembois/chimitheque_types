use crate::searchable::impl_searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SignalWord {
    #[serde(default)]
    pub match_exact_search: bool,
    pub signal_word_id: Option<u64>,
    pub signal_word_label: String,
}

impl_searchable!(SignalWord, table: "signal_word", id: signal_word_id, text: signal_word_label);

#[cfg(test)]
#[path = "signalword_tests.rs"]
mod signalword_tests;
