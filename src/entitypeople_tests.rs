#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::entitypeople::Entitypeople;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"entitypeople_entity_id": 1, "entitypeople_person_id": 2, "entitypeople_person_email": "manager@example.com", "entitypeople_entity_name": "lab"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Entitypeople>(&sample());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Entitypeople>(
            &sample(),
            &[
                "entitypeople_entity_id",
                "entitypeople_person_id",
                "entitypeople_person_email",
                "entitypeople_entity_name",
            ],
        );
    }
}
