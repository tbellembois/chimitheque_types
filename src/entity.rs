use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entity {
    pub entity_id: u64,
    pub entity_name: String,
}
