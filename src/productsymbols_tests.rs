#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productsymbols::Productsymbols;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"productsymbols_product_id": 1, "productsymbols_symbol_id": 2, "productsymbols_symbol_label": "GHS01"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Productsymbols>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Productsymbols>(
            &sample(),
            &[
                "productsymbols_product_id",
                "productsymbols_symbol_id",
                "productsymbols_symbol_label",
            ],
        );
    }
}
