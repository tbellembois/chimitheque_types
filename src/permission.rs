use crate::{error::ParseError, person::Person};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// What a permission allows on an item.
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone, Default)]
pub enum PermissionName {
    /// No access (`n`).
    #[default]
    #[serde(rename = "n")]
    None,
    /// Read access (`r`).
    #[serde(rename = "r")]
    Read,
    /// Read and write access (`w`).
    #[serde(rename = "w")]
    Write,
    /// Borrow access (`b`).
    #[serde(rename = "b")]
    Borrow,
    /// Full access (`all`).
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

// StoreLocations is not used by the backend as the policy to CRUD store locations
// are defined by policies on entities. See casbin policy definition.
// It is used by the frontend to check permissions on store locations with fake http requests.
/// What a permission applies to.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub enum PermissionItem {
    /// Every item (`all`).
    #[serde(rename = "all")]
    All,
    /// Entities (`entities`).
    #[serde(rename = "entities")]
    Entities,
    /// Products (`products`).
    #[default]
    #[serde(rename = "products")]
    Products,
    /// Restricted products (`rproducts`).
    #[serde(rename = "rproducts")]
    RestrictedProducts,
    /// Storages (`storages`).
    #[serde(rename = "storages")]
    Storages,
    /// Store locations (`store_locations`).
    #[serde(rename = "store_locations")]
    StoreLocations,
}

impl Display for PermissionItem {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            PermissionItem::All => write!(f, "all"),
            PermissionItem::Entities => write!(f, "entities"),
            PermissionItem::Products => write!(f, "products"),
            PermissionItem::RestrictedProducts => write!(f, "rproducts"),
            PermissionItem::Storages => write!(f, "storages"),
            PermissionItem::StoreLocations => write!(f, "store_locations"),
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
            "store_locations" => Ok(PermissionItem::StoreLocations),
            _ => Err(ParseError::ParsePermissionItemError(input.to_string())),
        }
    }
}

/// A person's permission on an item, optionally restricted to one entity.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Permission {
    /// Person the permission is granted to.
    pub person: Person,
    /// What the permission allows.
    pub permission_name: PermissionName,
    /// What the permission applies to.
    pub permission_item: PermissionItem,
    /// Entity the permission is restricted to; `None` for all entities.
    pub permission_entity: Option<u64>,
}

#[cfg(test)]
#[path = "permission_tests.rs"]
mod permission_tests;
