#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::personentities::Personentities;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"personentities_person_id": 2, "personentities_entity_id": 1, "personentities_person_email": "chemist@example.com", "personentities_entity_name": "lab"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Personentities>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Personentities>(
            &sample(),
            &[
                "personentities_person_id",
                "personentities_entity_id",
                "personentities_person_email",
                "personentities_entity_name",
            ],
        );
    }
}
