use crate::entity::Entity;
use chimitheque_utils::string::{clean, Transform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

impl StoreLocation {
    pub fn sanitize_and_validate(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.store_location_name = clean(&self.store_location_name, Transform::None);
        Ok(())
    }
}
