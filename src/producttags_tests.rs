#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::producttags::Producttags;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"producttags_product_id": 1, "producttags_tag_id": 2, "producttags_tag_label": "solvent"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Producttags>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_producttags_tag_id() {
        let mut json = sample();
        json.as_object_mut().unwrap().remove("producttags_tag_id");
        assert!(serde_json::from_value::<Producttags>(json).is_err());
    }
}
