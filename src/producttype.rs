use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::ParseError;

/// Kind of product.
#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone, Default)]
pub enum ProductType {
    /// A chemical product (`chem`).
    #[default]
    #[serde(rename = "chem")]
    Chem,
    /// A biological reagent (`bio`).
    #[serde(rename = "bio")]
    Bio,
    /// A laboratory consumable (`cons`).
    #[serde(rename = "cons")]
    Cons,
}

impl Display for ProductType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ProductType::Chem => write!(f, "chem"),
            ProductType::Bio => write!(f, "bio"),
            ProductType::Cons => write!(f, "cons"),
        }
    }
}

impl FromStr for ProductType {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<ProductType, Self::Err> {
        match input {
            "chem" => Ok(ProductType::Chem),
            "cons" => Ok(ProductType::Cons),
            "bio" => Ok(ProductType::Bio),
            _ => Err(ParseError::ParseProductTypeError(input.to_string())),
        }
    }
}

#[cfg(test)]
#[path = "producttype_tests.rs"]
mod producttype_tests;
