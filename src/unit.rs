use crate::unittype::UnitType;
use crate::validation::check_positive;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};

/// Represents a measurement unit with its properties and optional relationships.
///
/// # Fields
/// * `unit_id` - Unique identifier for the unit (None if not yet persisted)
/// * `unit_label` - Human-readable name of the unit (e.g., "gram", "milliliter")
/// * `unit_multiplier` - Conversion factor to base unit (1.0 for base units)
/// * `unit_type` - Categorization of the unit (mass, volume, etc.)
/// * `unit` - Optional reference to a parent/base unit for conversion hierarchy
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Unit {
    pub unit_id: Option<u64>,
    pub unit_label: String,
    pub unit_multiplier: f64,
    #[serde(default)]
    pub unit_type: UnitType,

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
