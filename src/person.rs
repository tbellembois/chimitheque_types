use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Person {
    pub person_id: u64,
    pub person_email: String,
}
