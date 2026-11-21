use crate::unittype::UnitType;
use crate::validation::check_positive;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};

/// A measurement unit, e.g. `mL`, possibly converting to a reference unit of
/// the same type, e.g. `L`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Unit {
    /// Database id; `None` for a new unit.
    pub unit_id: Option<u64>,
    /// Unit symbol, e.g. `mL` (case-sensitive).
    pub unit_label: String,
    /// Conversion factor to the reference unit of the same type.
    pub unit_multiplier: f64,
    /// What the unit measures.
    #[serde(default)]
    pub unit_type: UnitType,

    /// Reference unit this unit converts to, if any.
    pub unit: Option<Box<Unit>>,
}

impl Unit {
    /// Cleans the label and ensures it is not empty, and that the multiplier is
    /// a finite number greater than 0.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` for an empty label, or
    /// `ValidationError::OutOfRange` for an invalid multiplier.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // Unit labels are case-sensitive (mL, µM), so only whitespace is cleaned.
        require_text(&mut self.unit_label, "unit_label", Transform::None)?;
        check_positive(Some(self.unit_multiplier), "unit_multiplier")?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "unit_tests.rs"]
mod unit_tests;
