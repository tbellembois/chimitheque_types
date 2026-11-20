#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::{
        permission::{PermissionItem, PermissionName},
        person::*,
    };

    #[test]
    fn test_email_normalization_policy() {
        for (input, expected) in [
            ("User@EXAMPLE.COM", "user@example.com"),
            ("user@example.com", "user@example.com"),
            (" \tUser+Lab@Example.COM\r\n", "user+lab@example.com"),
            (r#""A  B"@EXAMPLE.COM"#, r#""a  b"@example.com"#),
            (r#""A@B"@EXAMPLE.COM"#, r#""a@b"@example.com"#),
            ("User@LOCALHOST", "user@localhost"),
            ("User@[127.0.0.1]", "user@[127.0.0.1]"),
        ] {
            let mut person = Person {
                person_email: input.to_string(),
                ..Default::default()
            };
            person.sanitize_and_validate().unwrap();
            assert_eq!(person.person_email, expected, "{input:?}");

            // Revalidating a normalized address must not change its identity.
            person.sanitize_and_validate().unwrap();
            assert_eq!(person.person_email, expected, "idempotence: {input:?}");
        }
    }

    #[test]
    fn test_email_normalization_preserves_distinct_quoted_whitespace() {
        // Lowercasing must not collapse significant whitespace inside quoted local parts.
        for (first, second) in [
            (r#""A  B"@example.com"#, r#""A B"@example.com"#),
            (r#"" A "@example.com"#, r#""A"@example.com"#),
        ] {
            let mut first_person = Person {
                person_email: first.to_string(),
                ..Default::default()
            };
            let mut second_person = Person {
                person_email: second.to_string(),
                ..Default::default()
            };
            first_person.sanitize_and_validate().unwrap();
            second_person.sanitize_and_validate().unwrap();
            assert_ne!(first_person.person_email, second_person.person_email);
        }
    }

    #[test]
    fn test_email_validation_rejects_invalid_input_without_mutating_it() {
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
            let mut person = Person {
                person_email: input.to_string(),
                ..Default::default()
            };
            assert!(person.sanitize_and_validate().is_err(), "{input:?}");
            assert_eq!(person.person_email, input, "invalid input was mutated");
        }
    }

    #[test]
    fn test_person_deserialization() {
        let json_data = r#"{
            "person_id": 1,
            "person_email": "admin@chimitheque.fr",
            "entities": [
                {
                    "entity_id": 29,
                    "entity_name": "Département de chimie",
                    "entity_description": null,
                    "managers": null,
                    "entity_nb_store_locations": null,
                    "entity_nb_people": null
                }
            ],
            "managed_entities": null,
            "permissions": [
                {
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
                    "permission_entity": null
                },
                {
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
                }
            ],
            "is_admin": true
        }"#;

        let person: Person = serde_json::from_str(json_data).unwrap();

        let expected_person = Person {
            person_id: Some(1),
            person_email: "admin@chimitheque.fr".to_string(),
            entities: Some(vec![Entity {
                entity_id: Some(29),
                entity_name: "Département de chimie".to_string(),
                ..Default::default()
            }]),
            permissions: Some(vec![
                Permission {
                    person: Person {
                        person_id: Some(1),
                        person_email: "admin@chimitheque.fr".to_string(),
                        ..Default::default()
                    },
                    permission_name: PermissionName::All,
                    permission_item: PermissionItem::All,
                    permission_entity: None,
                },
                Permission {
                    person: Person {
                        person_id: Some(1),
                        person_email: "admin@chimitheque.fr".to_string(),
                        ..Default::default()
                    },
                    permission_name: PermissionName::All,
                    permission_item: PermissionItem::All,
                    permission_entity: Some(29),
                },
            ]),
            is_admin: true,
            ..Default::default()
        };

        assert_eq!(person, expected_person);
    }
}
