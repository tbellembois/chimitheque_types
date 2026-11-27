use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::{ParseError, ValidationError};
use crate::unit::Unit;

/// Unit categories serialized as `snake_case` strings, matching `Display` and `FromStr`.
/// Legacy `PascalCase` JSON strings are also accepted during deserialization.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum UnitType {
    /// A quantity (volume, mass, length), e.g. `mL`, `g`.
    #[default]
    #[serde(alias = "Quantity")]
    Quantity,
    /// A concentration, e.g. `mg/L`, `%`.
    #[serde(alias = "Concentration")]
    Concentration,
    /// A temperature, e.g. `°C`.
    #[serde(alias = "Temperature")]
    Temperature,
    /// A molecular weight, e.g. `g/mol`.
    #[serde(alias = "MolecularWeight")]
    MolecularWeight,
}

impl UnitType {
    /// Rejects a set `unit` of `field` whose type, looked up from its id with
    /// `unit_type_of`, is not this unit type. A unit without an id or with an
    /// unknown id is rejected too.
    ///
    /// # Examples
    ///
    /// `check_unit` is private to the crate, so this example is not run as a
    /// doctest; `test_check_unit` covers the same cases.
    ///
    /// ```ignore
    /// // In the database, unit 1 is a temperature and unit 2 a quantity.
    /// let unit_type_of = |unit_id: u64| match unit_id {
    ///     1 => Some(UnitType::Temperature),
    ///     2 => Some(UnitType::Quantity),
    ///     _ => None,
    /// };
    /// let unit = |unit_id| Unit { unit_id, ..Default::default() };
    ///
    /// // 1. No unit: nothing to check.
    /// assert!(UnitType::Temperature.check_unit(None, "unit_temperature", &unit_type_of).is_ok());
    ///
    /// // 2. A temperature unit for a temperature: accepted.
    /// assert!(UnitType::Temperature
    ///     .check_unit(Some(&unit(Some(1))), "unit_temperature", &unit_type_of)
    ///     .is_ok());
    ///
    /// // 3. A quantity unit for a temperature: rejected.
    /// assert!(UnitType::Temperature
    ///     .check_unit(Some(&unit(Some(2))), "unit_temperature", &unit_type_of)
    ///     .is_err());
    ///
    /// // 4. An id unknown in the database: rejected.
    /// assert!(UnitType::Temperature
    ///     .check_unit(Some(&unit(Some(9))), "unit_temperature", &unit_type_of)
    ///     .is_err());
    ///
    /// // 5. A unit without an id: rejected, its type can not be looked up.
    /// assert!(UnitType::Temperature
    ///     .check_unit(Some(&unit(None)), "unit_temperature", &unit_type_of)
    ///     .is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidUnit` if the unit is rejected.
    pub(crate) fn check_unit(
        self,
        unit: Option<&Unit>,
        field: &'static str,
        unit_type_of: &impl Fn(u64) -> Option<UnitType>,
    ) -> Result<(), ValidationError> {
        let Some(unit) = unit else {
            return Ok(());
        };
        match unit.unit_id.and_then(unit_type_of) {
            Some(unit_type) if unit_type == self => Ok(()),
            _ => Err(ValidationError::InvalidUnit {
                field,
                expected: self,
            }),
        }
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            UnitType::Quantity => write!(f, "quantity"),
            UnitType::Concentration => write!(f, "concentration"),
            UnitType::Temperature => write!(f, "temperature"),
            UnitType::MolecularWeight => write!(f, "molecular_weight"),
        }
    }
}

impl FromStr for UnitType {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<UnitType, Self::Err> {
        match input {
            "quantity" => Ok(UnitType::Quantity),
            "concentration" => Ok(UnitType::Concentration),
            "temperature" => Ok(UnitType::Temperature),
            "molecular_weight" => Ok(UnitType::MolecularWeight),
            _ => Err(ParseError::ParseUnitTypeError(input.to_string())),
        }
    }
}

#[cfg(test)]
#[path = "unittype_tests.rs"]
mod unittype_tests;
