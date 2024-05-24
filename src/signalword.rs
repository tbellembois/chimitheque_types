use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Signalword {
    pub match_exact_search: bool,
    pub signalword_id: u64,
    pub signalword_label: String,
}
