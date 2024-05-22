use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub person_id: u64,
    pub person_email: String,
}
