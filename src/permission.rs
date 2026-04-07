use crate::{error::ParseError, person::Person};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone, Default)]
pub enum PermissionName {
    #[default]
    #[serde(rename = "n")]
    None,
    #[serde(rename = "r")]
    Read,
    #[serde(rename = "w")]
    Write,
    #[serde(rename = "b")]
    Borrow,
    #[serde(rename = "all")]
    All,
}

impl Display for PermissionName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            PermissionName::None => write!(f, "n"),
            PermissionName::Read => write!(f, "r"),
            PermissionName::Write => write!(f, "w"),
            PermissionName::Borrow => write!(f, "b"),
            PermissionName::All => write!(f, "all"),
        }
    }
}

impl FromStr for PermissionName {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<PermissionName, Self::Err> {
        match input {
            "n" => Ok(PermissionName::None),
            "r" => Ok(PermissionName::Read),
            "w" => Ok(PermissionName::Write),
            "b" => Ok(PermissionName::Borrow),
            "all" => Ok(PermissionName::All),
            _ => Err(ParseError::ParsePermissionNameError(input.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub enum PermissionItem {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "entities")]
    Entities,
    #[default]
    #[serde(rename = "products")]
    Products,
    #[serde(rename = "rproducts")]
    RestrictedProducts,
    #[serde(rename = "storages")]
    Storages,
}

impl Display for PermissionItem {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            PermissionItem::All => write!(f, "all"),
            PermissionItem::Entities => write!(f, "entities"),
            PermissionItem::Products => write!(f, "products"),
            PermissionItem::RestrictedProducts => write!(f, "rproducts"),
            PermissionItem::Storages => write!(f, "storages"),
        }
    }
}

impl FromStr for PermissionItem {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<PermissionItem, Self::Err> {
        match input {
            "all" => Ok(PermissionItem::All),
            "entities" => Ok(PermissionItem::Entities),
            "products" => Ok(PermissionItem::Products),
            "rproducts" => Ok(PermissionItem::RestrictedProducts),
            "storages" => Ok(PermissionItem::Storages),
            _ => Err(ParseError::ParsePermissionItemError(input.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Permission {
    pub person: Person,
    pub permission_name: PermissionName,
    pub permission_item: PermissionItem,
    pub permission_entity: i64,
}

#[cfg(test)]
#[path = "permission_tests.rs"]
mod permission_tests;
