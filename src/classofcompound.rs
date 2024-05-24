use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Classofcompound {
    pub match_exact_search: bool,
    pub classofcompound_id: u64,
    pub classofcompound_label: String,
}
