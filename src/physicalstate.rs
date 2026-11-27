use crate::searchable::impl_searchable;
use crate::{error::ValidationError, validation::clean_required_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};

/// A physical state, e.g. `liquid`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PhysicalState {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new physical state.
    pub physical_state_id: Option<u64>,
    /// Physical state label.
    pub physical_state_label: String,
}

impl PhysicalState {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The label is required.
        clean_required_text(
            &mut self.physical_state_label,
            "physical_state_label",
            Transform::None,
        )?;
        Ok(())
    }
}

impl_searchable!(PhysicalState, table: "physical_state", id: physical_state_id, text: physical_state_label);

#[cfg(test)]
#[path = "physicalstate_tests.rs"]
mod physicalstate_tests;
