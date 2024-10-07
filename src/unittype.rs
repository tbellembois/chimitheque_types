use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseUnitTypeError;

impl Display for ParseUnitTypeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "wrong unit type")
    }
}

impl std::error::Error for ParseUnitTypeError {}

impl FromStr for UnitType {
    type Err = ParseUnitTypeError;

    fn from_str(input: &str) -> Result<UnitType, Self::Err> {
        match input {
            "quantity" => Ok(UnitType::Quantity),
            "concentration" => Ok(UnitType::Concentration),
            "temperature" => Ok(UnitType::Temperature),
            "molecular_weight" => Ok(UnitType::MolecularWeight),
            _ => Err(ParseUnitTypeError),
        }
    }
}
