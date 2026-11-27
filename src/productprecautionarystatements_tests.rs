#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productprecautionarystatements::Productprecautionarystatements;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"productprecautionarystatements_product_id": 1, "productprecautionarystatements_precautionary_statement_id": 2, "productprecautionarystatements_precautionary_statement_label": "Wear protective gloves", "productprecautionarystatements_precautionary_statement_reference": "P280"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Productprecautionarystatements>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Productprecautionarystatements>(
            &sample(),
            &[
                "productprecautionarystatements_product_id",
                "productprecautionarystatements_precautionary_statement_id",
                "productprecautionarystatements_precautionary_statement_label",
                "productprecautionarystatements_precautionary_statement_reference",
            ],
        );
    }
}
