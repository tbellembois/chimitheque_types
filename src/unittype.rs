use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::ParseError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum UnitType {
    #[default]
    Quantity,
    Concentration,
    Temperature,
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
            _ => Err(ParseError::ParseUnitTypeError),
        }
    }
}

#[cfg(test)]
#[path = "unittype_tests.rs"]
mod unittype_tests;
