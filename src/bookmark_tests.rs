#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::bookmark::Bookmark;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"bookmark_id": 1, "person": 2, "product": 3})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Bookmark>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_product() {
        let mut json = sample();
        json.as_object_mut().unwrap().remove("product");
        assert!(serde_json::from_value::<Bookmark>(json).is_err());
    }
}
