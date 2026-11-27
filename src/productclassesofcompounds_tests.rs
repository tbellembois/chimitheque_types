#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productclassesofcompounds::Productclassesofcompounds;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"productclassesofcompounds_product_id": 1, "productclassesofcompounds_class_of_compound_id": 2, "productclassesofcompounds_class_of_compound_label": "alcohol"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Productclassesofcompounds>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Productclassesofcompounds>(
            &sample(),
            &[
                "productclassesofcompounds_product_id",
                "productclassesofcompounds_class_of_compound_id",
                "productclassesofcompounds_class_of_compound_label",
            ],
        );
    }
}
