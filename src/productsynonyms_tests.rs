#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productsynonyms::Productsynonyms;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"productsynonyms_product_id": 1, "productsynonyms_name_id": 2, "productsynonyms_name_label": "ETHYL ALCOHOL"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Productsynonyms>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_productsynonyms_name_id() {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("productsynonyms_name_id");
        assert!(serde_json::from_value::<Productsynonyms>(json).is_err());
    }
}
