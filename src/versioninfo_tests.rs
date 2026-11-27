#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use crate::versioninfo::VersionInfo;
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"version": "1.2.3", "build_time": "2026-11-21T10:00:00Z", "git_commit": "v1.2.3-4-gabcdef", "git_commit_hash": "abcdef0123456789", "target": "x86_64-unknown-linux-gnu", "rustc": "1.88.0"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<VersionInfo>(&sample());
    }

    #[test]
    fn test_serde_optional_fields() {
        // Missing optional fields are deserialized as None, and serialized as null.
        let mut json = sample();
        for field in ["git_commit", "git_commit_hash"] {
            json.as_object_mut().unwrap().remove(field);
        }
        let value: VersionInfo = serde_json::from_value(json).unwrap();
        assert_eq!(value.git_commit, None);
        assert_eq!(value.git_commit_hash, None);

        let mut json = sample();
        for field in ["git_commit", "git_commit_hash"] {
            json[field] = serde_json::Value::Null;
        }
        assert_serde_round_trip::<VersionInfo>(&json);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<VersionInfo>(
            &sample(),
            &["version", "build_time", "target", "rustc"],
        );
    }
}
