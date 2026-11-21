#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productprecautionarystatements::Productprecautionarystatements;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"productprecautionarystatements_product_id": 1, "productprecautionarystatements_precautionary_statement_id": 2, "productprecautionarystatements_precautionary_statement_label": "Wear protective gloves", "productprecautionarystatements_precautionary_statement_reference": "P280"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Productprecautionarystatements>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_productprecautionarystatements_precautionary_statement_reference()
    {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("productprecautionarystatements_precautionary_statement_reference");
        assert!(serde_json::from_value::<Productprecautionarystatements>(json).is_err());
    }
}
