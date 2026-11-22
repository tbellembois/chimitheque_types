#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::storelocation::*;

    #[test]
    fn test_sanitize_and_validate_leading_trailing_spaces() {
        let mut store_location = StoreLocation {
            store_location_id: Some(1),
            store_location_name: "  hello world  ".to_string(),
            store_location_can_store: false,
            store_location_color: None,
            store_location_full_path: None,
            store_location_nb_storages: None,
            store_location_nb_children: None,
            entity: None,
            store_location: None,
        };
        assert!(store_location.sanitize_and_validate().is_ok());
        assert_eq!(store_location.store_location_name, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_multiple_spaces() {
        let mut store_location = StoreLocation {
            store_location_id: Some(1),
            store_location_name: "  hello    world  ".to_string(),
            store_location_can_store: false,
            store_location_color: None,
            store_location_full_path: None,
            store_location_nb_storages: None,
            store_location_nb_children: None,
            entity: None,
            store_location: None,
        };
        assert!(store_location.sanitize_and_validate().is_ok());
        assert_eq!(store_location.store_location_name, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_empty_label() {
        let mut store_location = StoreLocation {
            store_location_id: Some(1),
            store_location_name: String::default(),
            store_location_can_store: false,
            store_location_color: None,
            store_location_full_path: None,
            store_location_nb_storages: None,
            store_location_nb_children: None,
            entity: None,
            store_location: None,
        };
        assert!(store_location.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_no_spaces() {
        let mut store_location = StoreLocation {
            store_location_id: Some(1),
            store_location_name: "hello world".to_string(),
            store_location_can_store: false,
            store_location_color: None,
            store_location_full_path: None,
            store_location_nb_storages: None,
            store_location_nb_children: None,
            entity: None,
            store_location: None,
        };
        assert!(store_location.sanitize_and_validate().is_ok());
        assert_eq!(store_location.store_location_name, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_color() {
        let mut value = StoreLocation {
            store_location_name: "shelf".to_string(),
            store_location_color: Some(" #ff0000 ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.store_location_color.as_deref(), Some("#ff0000"));

        value.store_location_color = Some("   ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.store_location_color, None);
    }

    #[test]
    fn test_sanitize_and_validate_color_format() {
        for color in ["red", "#ff00", "ff0000", "#gg0000"] {
            let mut value = StoreLocation {
                store_location_name: "shelf".to_string(),
                store_location_color: Some(color.to_string()),
                ..Default::default()
            };
            let error = value.sanitize_and_validate().unwrap_err();
            assert!(
                matches!(
                    error,
                    crate::error::ValidationError::InvalidFormat {
                        field: "store_location_color",
                        ..
                    }
                ),
                "{color}"
            );
        }
    }

    #[test]
    fn test_sanitize_and_validate_parent() {
        let parent = |id| {
            Some(Box::new(StoreLocation {
                store_location_id: Some(id),
                ..Default::default()
            }))
        };
        let mut value = StoreLocation {
            store_location_id: Some(1),
            store_location_name: "shelf".to_string(),
            store_location: parent(2),
            ..Default::default()
        };
        assert!(value.sanitize_and_validate().is_ok());

        value.store_location = parent(1);
        assert!(matches!(
            value.sanitize_and_validate().unwrap_err(),
            crate::error::ValidationError::OutOfRange {
                field: "store_location",
                ..
            }
        ));
    }
}
