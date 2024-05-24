use crate::producer::Producer;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Producerref {
    pub match_exact_search: bool,
    pub producerref_id: u64,
    pub producerref_label: String,

    pub producer: Producer,
}
