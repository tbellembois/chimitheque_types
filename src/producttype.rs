use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::ParseError;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ProductType {
    #[default]
    chem,
    bio,
    cons,
}

impl Display for ProductType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ProductType::chem => write!(f, "chem"),
            ProductType::bio => write!(f, "bio"),
            ProductType::cons => write!(f, "cons"),
        }
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct ParseProductTypeError;
//
// impl Display for ParseProductTypeError {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "wrong product type")
//     }
// }
//
// impl std::error::Error for ParseProductTypeError {}

impl FromStr for ProductType {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<ProductType, Self::Err> {
        match input {
            "chem" => Ok(ProductType::chem),
            "cons" => Ok(ProductType::cons),
            "bio" => Ok(ProductType::bio),
            _ => Err(ParseError::ParseProductTypeError),
        }
    }
}
