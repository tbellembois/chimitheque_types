use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Tag {
    pub match_exact_search: bool,
    pub tag_id: u64,
    pub tag_label: String,
}
