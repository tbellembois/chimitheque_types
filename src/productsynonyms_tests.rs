#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::productsynonyms::Productsynonyms;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"productsynonyms_product_id": 1, "productsynonyms_name_id": 2, "productsynonyms_name_label": "ETHYL ALCOHOL"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Productsynonyms>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Productsynonyms>(
            &sample(),
            &[
                "productsynonyms_product_id",
                "productsynonyms_name_id",
                "productsynonyms_name_label",
            ],
        );
    }
}
