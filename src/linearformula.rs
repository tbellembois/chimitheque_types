use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Linearformula {
    pub match_exact_search: bool,
    pub linearformula_id: u64,
    pub linearformula_label: String,
}
