use crate::producer::Producer;
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
