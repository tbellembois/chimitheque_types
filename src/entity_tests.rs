#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::entity::*;
    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_serde_rejects_missing, assert_serde_round_trip, assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "entity_id": 1,
            "entity_name": "lab",
            "entity_description": "first floor",
            "managers": [{
                "person_id": 2,
                "person_email": "manager@example.com",
                "entities": null,
                "managed_entities": null,
                "permissions": null,
                "is_admin": false
            }],
            "entity_nb_store_locations": 3,
            "entity_nb_people": 4
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<Entity>(&sample());
        assert_eq!(value.managers.unwrap()[0].person_id, Some(2));
    }

    #[test]
    fn test_serde_optional_fields() {
        let value: Entity = serde_json::from_value(json!({"entity_name": "lab"})).unwrap();
        assert_eq!(value.entity_id, None);
        assert_eq!(value.entity_description, None);
        assert!(value.managers.is_none());
        assert_eq!(value.entity_nb_store_locations, None);
        assert_eq!(value.entity_nb_people, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Entity>(&sample(), &["entity_name"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = Entity {
            entity_name: "  lab   one  ".to_string(),
            entity_description: Some("  first   floor  ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.entity_name, "lab one");
        assert_eq!(value.entity_description.as_deref(), Some("first floor"));

        // An empty description becomes None.
        value.entity_description = Some("   ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.entity_description, None);
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "   "] {
            let mut value = Entity {
                entity_name: empty.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "entity_name");
            assert!(matches!(error, ValidationError::Empty { .. }));
        }
    }
}
