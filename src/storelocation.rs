use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Storelocation {
    pub storelocation_id: u64,
    pub storelocation_name: String,
    pub storelocation_canstore: bool,
    pub storelocation_color: Option<String>,
    pub storelocation_fullpath: Option<String>,

    pub entity: Option<Entity>,
    pub storelocation: Option<Box<Storelocation>>,
}
