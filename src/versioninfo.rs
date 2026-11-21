use serde::{Deserialize, Serialize};

/// Build information of the running application.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VersionInfo {
    /// Application version.
    pub version: String,
    /// Build date and time.
    pub build_time: String,
    /// Git commit description, if known.
    pub git_commit: Option<String>,
    /// Git commit hash, if known.
    pub git_commit_hash: Option<String>,
    /// Target platform triple.
    pub target: String,
    /// Version of the Rust compiler used.
    pub rustc: String,
}

#[cfg(test)]
#[path = "versioninfo_tests.rs"]
mod versioninfo_tests;
