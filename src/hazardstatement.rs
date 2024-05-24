use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Hazardstatement {
    pub match_exact_search: bool,
    pub hazardstatement_id: u64,
    pub hazardstatement_label: String,
    pub hazardstatement_reference: String,
}
