#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::producttags::Producttags;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"producttags_product_id": 1, "producttags_tag_id": 2, "producttags_tag_label": "solvent"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Producttags>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Producttags>(
            &sample(),
            &[
                "producttags_product_id",
                "producttags_tag_id",
                "producttags_tag_label",
            ],
        );
    }
}
