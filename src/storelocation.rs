use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreLocation {
    pub store_location_id: Option<u64>,
    pub store_location_name: String,
    pub store_location_can_store: bool,
    pub store_location_color: Option<String>,

    // Computed field on insert/update.
    pub store_location_full_path: Option<String>,

    // Computed fields on select, not in DB.
    pub store_location_nb_storages: Option<u64>,
    pub store_location_nb_children: Option<u64>,

    pub entity: Option<Entity>,
    pub store_location: Option<Box<StoreLocation>>,
}

impl Default for StoreLocation {
    fn default() -> Self {
        StoreLocation {
            store_location_id: None,
            store_location_name: "".to_string(),
            store_location_can_store: false,
            store_location_color: None,
            store_location_full_path: None,
            store_location_nb_storages: None,
            store_location_nb_children: None,
            entity: None,
            store_location: None,
        }
    }
}
