use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Symbol {
    pub match_exact_search: bool,
    pub symbol_id: u64,
    pub symbol_label: String,
}
