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
}
