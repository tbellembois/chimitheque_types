#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::versioninfo::VersionInfo;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"version": "1.2.3", "build_time": "2026-11-21T10:00:00Z", "git_commit": "v1.2.3-4-gabcdef", "git_commit_hash": "abcdef0123456789", "target": "x86_64-unknown-linux-gnu", "rustc": "1.88.0"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<VersionInfo>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_version() {
        let mut json = sample();
        json.as_object_mut().unwrap().remove("version");
        assert!(serde_json::from_value::<VersionInfo>(json).is_err());
    }

    #[test]
    fn test_serde_git_fields_are_optional() {
        let mut json = sample();
        json["git_commit"] = serde_json::Value::Null;
        json["git_commit_hash"] = serde_json::Value::Null;
        assert_round_trip::<VersionInfo>(&json);
    }
}
