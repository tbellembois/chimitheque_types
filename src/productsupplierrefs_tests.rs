#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productsupplierrefs::Productsupplierrefs;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"productsupplierrefs_product_id": 1, "productsupplierrefs_supplier_ref_id": 2, "productsupplierrefs_supplier_ref_label": "S-123", "productsupplierrefs_supplier_id": 3, "productsupplierrefs_supplier_label": "Acme"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Productsupplierrefs>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Productsupplierrefs>(
            &sample(),
            &[
                "productsupplierrefs_product_id",
                "productsupplierrefs_supplier_ref_id",
                "productsupplierrefs_supplier_ref_label",
                "productsupplierrefs_supplier_id",
                "productsupplierrefs_supplier_label",
            ],
        );
    }
}
