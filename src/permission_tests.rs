#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::permission::*;
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    const NAMES: [(PermissionName, &str); 5] = [
        (PermissionName::None, "n"),
        (PermissionName::Read, "r"),
        (PermissionName::Write, "w"),
        (PermissionName::Borrow, "b"),
        (PermissionName::All, "all"),
    ];

    const ITEMS: [(PermissionItem, &str); 6] = [
        (PermissionItem::All, "all"),
        (PermissionItem::Entities, "entities"),
        (PermissionItem::Products, "products"),
        (PermissionItem::RestrictedProducts, "rproducts"),
        (PermissionItem::Storages, "storages"),
        (PermissionItem::StoreLocations, "store_locations"),
    ];

    fn sample() -> serde_json::Value {
        json!({
            "person": {
                "person_id": 1,
                "person_email": "chemist@example.com",
                "entities": null,
                "managed_entities": null,
                "permissions": null,
                "is_admin": false
            },
            "permission_name": "w",
            "permission_item": "rproducts",
            "permission_entity": 2
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<Permission>(&sample());
        assert_eq!(value.person.person_id, Some(1));
        assert_eq!(value.permission_name, PermissionName::Write);
        assert_eq!(value.permission_item, PermissionItem::RestrictedProducts);
        assert_eq!(value.permission_entity, Some(2));

        for (name, text) in NAMES {
            assert_eq!(serde_json::to_value(name).unwrap(), json!(text));
            assert_eq!(
                serde_json::from_value::<PermissionName>(json!(text)).unwrap(),
                name
            );
        }
        for (item, text) in ITEMS {
            assert_eq!(serde_json::to_value(&item).unwrap(), json!(text));
            assert_eq!(
                serde_json::from_value::<PermissionItem>(json!(text)).unwrap(),
                item
            );
        }
    }

    #[test]
    fn test_serde_optional_fields() {
        let mut json = sample();
        json.as_object_mut().unwrap().remove("permission_entity");
        let value: Permission = serde_json::from_value(json).unwrap();
        assert_eq!(value.permission_entity, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Permission>(
            &sample(),
            &["person", "permission_name", "permission_item"],
        );
    }

    #[test]
    fn test_serde_rejects_invalid() {
        for text in ["", "read", "R", "none"] {
            assert!(serde_json::from_value::<PermissionName>(json!(text)).is_err());
        }
        for text in ["", "product", "rproduct", "Store_locations"] {
            assert!(serde_json::from_value::<PermissionItem>(json!(text)).is_err());
        }
    }

    // Other functions.

    #[test]
    fn test_default() {
        assert_eq!(PermissionName::default(), PermissionName::None);
        assert_eq!(PermissionItem::default(), PermissionItem::Products);
    }

    #[test]
    fn test_display_and_from_str() {
        for (name, text) in NAMES {
            assert_eq!(name.to_string(), text);
            assert_eq!(text.parse::<PermissionName>().unwrap(), name);
        }
        for (item, text) in ITEMS {
            assert_eq!(item.to_string(), text);
            assert_eq!(text.parse::<PermissionItem>().unwrap(), item);
        }
    }

    #[test]
    fn test_from_str_rejects_invalid() {
        for input in [
            "invalid", "123", "", "a", "read", "write", "borrow", "none", "All", "Read", "Write",
            "Borrow",
        ] {
            assert!(input.parse::<PermissionName>().is_err(), "{input}");
        }
        for input in [
            "invalid",
            "123",
            "",
            "a",
            "allitems",
            "entity",
            "product",
            "storagess",
            "rproduct",
        ] {
            assert!(input.parse::<PermissionItem>().is_err(), "{input}");
        }
    }
}
