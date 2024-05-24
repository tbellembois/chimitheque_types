use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Empiricalformula {
    pub match_exact_search: bool,
    pub empiricalformula_id: u64,
    pub empiricalformula_label: String,
}
