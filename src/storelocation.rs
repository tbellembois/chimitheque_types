//! Storage location module containing the `StoreLocation` struct and its implementations.
//! Represents physical or logical locations where chemical products can be stored.
//! Supports hierarchical storage organization through parent-child relationships.

use std::fmt;

use crate::entity::Entity;
use crate::error::ValidationError;
use chimitheque_defines::HTML_COLOR_RE;
use chimitheque_utils::string::{Transform, clean, clean_optional_text};
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
    /// Cleans the location name and ensures it's not empty, and that the store
    /// location is not its own parent. Set `store_location_id` before calling
    /// it when updating.
    ///
    /// # Returns
    ///
    /// `Ok(())` if validation succeeds, or an error if validation fails.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        // The name is required.
        self.store_location_name = clean(&self.store_location_name, Transform::None);
        if self.store_location_name.is_empty() {
            return Err(ValidationError::Empty {
                field: "store_location_name",
            });
        }
        // The color is optional but must be an HTML color.
        clean_optional_text(&mut self.store_location_color);
        if let Some(color) = self.store_location_color.as_deref()
            && !HTML_COLOR_RE.is_match(color)
        {
            return Err(ValidationError::InvalidFormat {
                field: "store_location_color",
                expected: "a color like #RRGGBB",
            });
        }
        // A store location can not be its own parent.
        if let (Some(id), Some(parent)) = (self.store_location_id, &self.store_location)
            && parent.store_location_id == Some(id)
        {
            return Err(ValidationError::OutOfRange {
                field: "store_location",
                expected: "a store location other than itself",
            });
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "storelocation_tests.rs"]
mod storelocation_tests;
