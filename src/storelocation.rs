use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreLocation {
    pub store_location_id: u64,
    pub store_location_name: String,
    pub store_location_can_store: bool,
    pub store_location_color: Option<String>,
    pub store_location_full_path: Option<String>,

    pub entity: Option<Entity>,
    pub store_location: Option<Box<StoreLocation>>,
}
