use crate::searchable::impl_searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PhysicalState {
    #[serde(default)]
    pub match_exact_search: bool,
    pub physical_state_id: Option<u64>,
    pub physical_state_label: String,
}

impl_searchable!(PhysicalState, table: "physical_state", id: physical_state_id, text: physical_state_label);

#[cfg(test)]
#[path = "physicalstate_tests.rs"]
mod physicalstate_tests;
