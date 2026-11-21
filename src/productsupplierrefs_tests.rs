#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productsupplierrefs::Productsupplierrefs;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"productsupplierrefs_product_id": 1, "productsupplierrefs_supplier_ref_id": 2, "productsupplierrefs_supplier_ref_label": "S-123", "productsupplierrefs_supplier_id": 3, "productsupplierrefs_supplier_label": "Acme"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Productsupplierrefs>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_productsupplierrefs_supplier_id() {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("productsupplierrefs_supplier_id");
        assert!(serde_json::from_value::<Productsupplierrefs>(json).is_err());
    }
}
