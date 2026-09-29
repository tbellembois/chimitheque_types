use crate::unittype::UnitType;
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
