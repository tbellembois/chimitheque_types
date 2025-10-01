use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bookmark {
    pub bookmark_id: Option<u64>,
    pub person: u64,
    pub product: u64,
}
