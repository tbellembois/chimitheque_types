use crate::producer::Producer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProducerRef {
    pub match_exact_search: bool,
    pub producer_ref_id: Option<u64>,
    pub producer_ref_label: String,

    pub producer: Producer,
}
