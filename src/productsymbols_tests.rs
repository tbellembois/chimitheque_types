#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productsymbols::Productsymbols;

    /// Deserializes `json` into `T` and serializes it back unchanged.
    fn assert_round_trip<T: serde::Serialize + serde::de::DeserializeOwned>(
        json: &serde_json::Value,
    ) {
        let value: T = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(&serde_json::to_value(&value).unwrap(), json);
    }

    fn sample() -> serde_json::Value {
        serde_json::json!({"productsymbols_product_id": 1, "productsymbols_symbol_id": 2, "productsymbols_symbol_label": "GHS01"})
    }

    #[test]
    fn test_serde_round_trip() {
        assert_round_trip::<Productsymbols>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_productsymbols_symbol_id() {
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("productsymbols_symbol_id");
        assert!(serde_json::from_value::<Productsymbols>(json).is_err());
    }
}
