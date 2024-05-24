use crate::unittype::UnitType;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Unit {
    pub unit_id: u64,
    pub unit_label: String,
    pub unit_multiplier: f64,
    pub unit_type: UnitType,

    pub unit: Option<Box<Unit>>,
}
