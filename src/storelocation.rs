/// Storage location module containing the `StoreLocation` struct and its implementations.
/// Represents physical or logical locations where chemical products can be stored.
/// Supports hierarchical storage organization through parent-child relationships.
use std::fmt;

use crate::entity::Entity;
use chimitheque_utils::string::{Transform, clean};
use serde::{Deserialize, Serialize};

/// Main `StoreLocation` structure representing a storage location.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreLocation {
    /// Unique identifier for the storage location.
    pub store_location_id: Option<u64>,
    /// Name/identifier of the storage location.
    pub store_location_name: String,
    /// Flag indicating if this location can store products.
    pub store_location_can_store: bool,
    /// Optional color code for visual identification.
    pub store_location_color: Option<String>,

    // Computed field on insert/update, not in DB.
    /// Full path representation of the location hierarchy.
    pub store_location_full_path: Option<String>,

    // Computed fields on select, not in DB.
    /// Number of products currently stored in this location.
    pub store_location_nb_storages: Option<u64>,
    /// Number of child locations under this location.
    pub store_location_nb_children: Option<u64>,

    /// Entity that owns/manages this storage location.
    pub entity: Option<Entity>,
    /// Parent storage location (for hierarchical organization).
    pub store_location: Option<Box<StoreLocation>>,
}

impl fmt::Display for StoreLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StoreLocation {{ store_location_id: {:?}, store_location_name: {} }}",
            self.store_location_id, self.store_location_name
        )
    }
}

impl StoreLocation {
    /// Validates and sanitizes all fields of the `StoreLocation`.
    ///
    /// Cleans the location name and ensures it's not empty.
    ///
    /// # Returns
    ///
    /// `Ok(())` if validation succeeds, or an error if validation fails.
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.store_location_name = clean(&self.store_location_name, Transform::None);
        if self.store_location_name.is_empty() {
            return Err(Box::new(crate::error::ParseError::EmptyInput));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "storelocation_tests.rs"]
mod storelocation_tests;
