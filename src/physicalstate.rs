use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Physicalstate {
    pub match_exact_search: bool,
    pub physicalstate_id: u64,
    pub physicalstate_label: String,
}
