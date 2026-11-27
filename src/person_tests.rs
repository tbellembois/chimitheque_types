#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::permission::{PermissionItem, PermissionName};
    use crate::person::*;
    use crate::test_helpers::{
        assert_serde_rejects_missing, assert_serde_round_trip, assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "person_id": 1,
            "person_email": "admin@chimitheque.fr",
            "entities": [{
                "entity_id": 29,
                "entity_name": "Département de chimie",
                "entity_description": null,
                "managers": null,
                "entity_nb_store_locations": null,
                "entity_nb_people": null
            }],
            "managed_entities": null,
            "permissions": [{
                "person": {
                    "person_id": 1,
                    "person_email": "admin@chimitheque.fr",
                    "entities": null,
                    "managed_entities": null,
                    "permissions": null,
                    "is_admin": false
                },
                "permission_name": "all",
                "permission_item": "all",
                "permission_entity": 29
            }],
            "is_admin": true
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<Person>(&sample());
        assert_eq!(value.person_id, Some(1));
        assert!(value.is_admin);
        assert_eq!(
            value.entities.unwrap()[0].entity_name,
            "Département de chimie"
        );
        let permission = &value.permissions.unwrap()[0];
        assert_eq!(permission.permission_name, PermissionName::All);
        assert_eq!(permission.permission_item, PermissionItem::All);
        assert_eq!(permission.permission_entity, Some(29));
    }

    #[test]
    fn test_serde_defaults() {
        let value: Person =
            serde_json::from_value(json!({"person_email": "chemist@example.com"})).unwrap();
        assert_eq!(value.person_id, None);
        assert!(value.entities.is_none());
        assert!(value.managed_entities.is_none());
        assert!(value.permissions.is_none());
        assert!(!value.is_admin);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Person>(&sample(), &["person_email"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        for (input, expected) in [
            ("User@EXAMPLE.COM", "user@example.com"),
            ("user@example.com", "user@example.com"),
            (" \tUser+Lab@Example.COM\r\n", "user+lab@example.com"),
            (r#""A  B"@EXAMPLE.COM"#, r#""a  b"@example.com"#),
            (r#""A@B"@EXAMPLE.COM"#, r#""a@b"@example.com"#),
            ("User@LOCALHOST", "user@localhost"),
            ("User@[127.0.0.1]", "user@[127.0.0.1]"),
        ] {
            let mut value = Person {
                person_email: input.to_string(),
                ..Default::default()
            };
            value.sanitize_and_validate().unwrap();
            assert_eq!(value.person_email, expected, "{input:?}");

            // Revalidating a normalized address must not change it.
            value.sanitize_and_validate().unwrap();
            assert_eq!(value.person_email, expected, "idempotence: {input:?}");
        }
    }

    #[test]
    fn test_sanitize_and_validate_keeps_quoted_whitespace() {
        // Lowercasing must not collapse significant whitespace inside quoted
        // local parts.
        for (first, second) in [
            (r#""A  B"@example.com"#, r#""A B"@example.com"#),
            (r#"" A "@example.com"#, r#""A"@example.com"#),
        ] {
            let mut first = Person {
                person_email: first.to_string(),
                ..Default::default()
            };
            let mut second = Person {
                person_email: second.to_string(),
                ..Default::default()
            };
            first.sanitize_and_validate().unwrap();
            second.sanitize_and_validate().unwrap();
            assert_ne!(first.person_email, second.person_email);
        }
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for input in [
            "",
            " \t\r\n",
            "not an email",
            "Alice <alice@example.com>",
            "<alice@example.com>",
            "User Name@EXAMPLE.COM",
            "User@EXAMPLE .COM",
            "User\nName@example.com",
            "User@",
            "@example.com",
            "first@example.com,second@example.com",
        ] {
            let mut value = Person {
                person_email: input.to_string(),
                ..Default::default()
            };
            assert_validation_error(value.sanitize_and_validate(), "person_email");
            // An invalid address is left unchanged.
            assert_eq!(value.person_email, input);
        }
    }
}
