#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::producthazardstatements::Producthazardstatements;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"producthazardstatements_product_id": 1, "producthazardstatements_hazard_statement_id": 2, "producthazardstatements_hazard_statement_label": "May cause cancer", "producthazardstatements_hazard_statement_reference": "H350", "producthazardstatements_hazard_statement_cmr": "C1A"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Producthazardstatements>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_producthazardstatements_hazard_statement_reference() {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("producthazardstatements_hazard_statement_reference");
        assert!(serde_json::from_value::<Producthazardstatements>(json).is_err());
    }

    #[test]
    fn test_serde_cmr_is_optional() {
        let mut json = sample();
        json["producthazardstatements_hazard_statement_cmr"] = serde_json::Value::Null;
        assert_round_trip::<Producthazardstatements>(&json);
    }
}
