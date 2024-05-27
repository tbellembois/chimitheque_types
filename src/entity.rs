use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub entity_id: u64,
    pub entity_name: String,
}
