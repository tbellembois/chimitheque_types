use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Casnumber {
    pub match_exact_search: bool,
    pub casnumber_id: u64,
    pub casnumber_label: String,
}
