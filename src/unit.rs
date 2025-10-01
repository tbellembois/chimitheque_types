use crate::unittype::UnitType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Unit {
    pub unit_id: Option<u64>,
    pub unit_label: String,
    pub unit_multiplier: f64,
    pub unit_type: UnitType,

    pub unit: Option<Box<Unit>>,
}
