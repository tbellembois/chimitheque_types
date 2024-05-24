use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Name {
    pub match_exact_search: bool,
    pub name_id: u64,
    pub name_label: String,
}
