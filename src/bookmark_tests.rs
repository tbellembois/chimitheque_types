#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::bookmark::Bookmark;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"bookmark_id": 1, "person": 2, "product": 3})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Bookmark>(&sample());
    }

    #[test]
    fn test_serde_optional_fields() {
        // Missing optional fields are deserialized as None, and serialized as null.
        let mut json = sample();
        json.as_object_mut().unwrap().remove("bookmark_id");
        let value: Bookmark = serde_json::from_value(json).unwrap();
        assert_eq!(value.bookmark_id, None);

        let mut json = sample();
        json["bookmark_id"] = serde_json::Value::Null;
        assert_serde_round_trip::<Bookmark>(&json);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Bookmark>(&sample(), &["person", "product"]);
    }
}
