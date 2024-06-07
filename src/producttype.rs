use serde::{Deserialize, Serialize};
use std::{
    default,
    fmt::{Display, Formatter},
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ProductType {
    #[default]
    Chemical,
    Biological,
    Consumable,
}

impl Display for ProductType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ProductType::Chemical => write!(f, "chemical"),
            ProductType::Biological => write!(f, "biological"),
            ProductType::Consumable => write!(f, "consumable"),
        }
    }
}
