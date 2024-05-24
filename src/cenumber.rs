use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Cenumber {
    pub match_exact_search: bool,
    pub cenumber_id: u64,
    pub cenumber_label: String,
}
