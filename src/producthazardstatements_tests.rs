#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::producthazardstatements::Producthazardstatements;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"producthazardstatements_product_id": 1, "producthazardstatements_hazard_statement_id": 2, "producthazardstatements_hazard_statement_label": "May cause cancer", "producthazardstatements_hazard_statement_reference": "H350", "producthazardstatements_hazard_statement_cmr": "C1A"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Producthazardstatements>(&sample());
    }

    #[test]
    fn test_serde_optional_fields() {
        // Missing optional fields are deserialized as None, and serialized as null.
        let mut json = sample();
        json.as_object_mut()
            .unwrap()
            .remove("producthazardstatements_hazard_statement_cmr");
        let value: Producthazardstatements = serde_json::from_value(json).unwrap();
        assert_eq!(value.producthazardstatements_hazard_statement_cmr, None);

        let mut json = sample();
        json["producthazardstatements_hazard_statement_cmr"] = serde_json::Value::Null;
        assert_serde_round_trip::<Producthazardstatements>(&json);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Producthazardstatements>(
            &sample(),
            &[
                "producthazardstatements_product_id",
                "producthazardstatements_hazard_statement_id",
                "producthazardstatements_hazard_statement_label",
                "producthazardstatements_hazard_statement_reference",
            ],
        );
    }
}
