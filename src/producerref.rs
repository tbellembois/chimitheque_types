use crate::producer::Producer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Producerref {
    pub match_exact_search: bool,
    pub producerref_id: u64,
    pub producerref_label: String,

    pub producer: Producer,
}
