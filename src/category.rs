use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Category {
    pub match_exact_search: bool,
    pub category_id: u64,
    pub category_label: String,
}
