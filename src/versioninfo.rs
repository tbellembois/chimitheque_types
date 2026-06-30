use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VersionInfo {
    pub version: String,
    pub build_time: String,
    pub git_commit: Option<String>,
    pub git_commit_hash: Option<String>,
    pub target: String,
    pub rustc: String,
}
