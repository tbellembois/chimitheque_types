use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::ParseError;

/// Unit categories serialized as `snake_case` strings, matching `Display` and `FromStr`.
/// Legacy `PascalCase` JSON strings are also accepted during deserialization.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum UnitType {
    #[default]
    #[serde(alias = "Quantity")]
    Quantity,
    #[serde(alias = "Concentration")]
    Concentration,
    #[serde(alias = "Temperature")]
    Temperature,
    #[serde(alias = "MolecularWeight")]
    MolecularWeight,
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
