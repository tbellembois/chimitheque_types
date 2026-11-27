#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::entity::Entity;
    use crate::error::ValidationError;
    use crate::storelocation::*;
    use crate::test_helpers::{
        assert_serde_rejects_missing, assert_validation_error, assert_value_round_trip,
    };

    fn sample() -> StoreLocation {
        StoreLocation {
            store_location_id: Some(2),
            store_location_name: "shelf".to_string(),
            store_location_can_store: true,
            store_location_color: Some("#ff0000".to_string()),
            store_location_full_path: Some("room/shelf".to_string()),
            store_location_nb_storages: Some(3),
            store_location_nb_children: Some(0),
            entity: Some(Entity {
                entity_id: Some(4),
                entity_name: "lab".to_string(),
                ..Default::default()
            }),
            store_location: Some(Box::new(StoreLocation {
                store_location_id: Some(1),
                store_location_name: "room".to_string(),
                ..Default::default()
            })),
        }
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let json = assert_value_round_trip(&sample());
        assert_eq!(json["store_location_name"], "shelf");
        assert_eq!(json["store_location_can_store"], true);
        assert_eq!(json["store_location_color"], "#ff0000");
        assert_eq!(json["entity"]["entity_id"], 4);
        assert_eq!(json["store_location"]["store_location_name"], "room");
    }

    #[test]
    fn test_serde_optional_fields() {
        let value: StoreLocation = serde_json::from_value(serde_json::json!({
            "store_location_name": "shelf",
            "store_location_can_store": false
        }))
        .unwrap();
        assert_eq!(value.store_location_id, None);
        assert_eq!(value.store_location_color, None);
        assert_eq!(value.store_location_full_path, None);
        assert_eq!(value.store_location_nb_storages, None);
        assert_eq!(value.store_location_nb_children, None);
        assert!(value.entity.is_none());
        assert!(value.store_location.is_none());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        let json = assert_value_round_trip(&sample());
        assert_serde_rejects_missing::<StoreLocation>(
            &json,
            &["store_location_name", "store_location_can_store"],
        );
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = StoreLocation {
            store_location_id: Some(1),
            store_location_name: "  hello    world  ".to_string(),
            store_location_color: Some(" #ff0000 ".to_string()),
            store_location: Some(Box::new(StoreLocation {
                store_location_id: Some(2),
                ..Default::default()
            })),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.store_location_name, "hello world");
        assert_eq!(value.store_location_color.as_deref(), Some("#ff0000"));

        // An empty color becomes None.
        value.store_location_color = Some("   ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.store_location_color, None);
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "   "] {
            let mut value = StoreLocation {
                store_location_name: empty.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "store_location_name");
            assert!(matches!(error, ValidationError::Empty { .. }));
        }

        for color in ["red", "#ff00", "ff0000", "#gg0000"] {
            let mut value = StoreLocation {
                store_location_name: "shelf".to_string(),
                store_location_color: Some(color.to_string()),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "store_location_color");
            assert!(
                matches!(error, ValidationError::InvalidFormat { .. }),
                "{color}"
            );
        }

        // A store location can not be its own parent.
        let mut value = StoreLocation {
            store_location_id: Some(1),
            store_location_name: "shelf".to_string(),
            store_location: Some(Box::new(StoreLocation {
                store_location_id: Some(1),
                ..Default::default()
            })),
            ..Default::default()
        };
        let error = assert_validation_error(value.sanitize_and_validate(), "store_location");
        assert!(matches!(error, ValidationError::OutOfRange { .. }));
    }
}
