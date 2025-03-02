use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::ParseError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ProductType {
    #[default]
    Chem,
    Bio,
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
            _ => Err(ParseError::ParseProductTypeError),
        }
    }
}
