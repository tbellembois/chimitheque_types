use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Borrowing {
    pub borrowing_id: Option<u64>,
    pub borrowing_comment: Option<String>,
    pub person: u64,
    pub storage: u64,
    pub borrower: u64,
}
