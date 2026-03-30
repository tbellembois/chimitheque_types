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
                    "permission_entity": -1
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
                    permission_entity: -1,
                },
                Permission {
                    person: Person {
                        person_id: Some(1),
                        person_email: "admin@chimitheque.fr".to_string(),
                        ..Default::default()
                    },
                    permission_name: PermissionName::All,
                    permission_item: PermissionItem::All,
                    permission_entity: 29,
                },
            ]),
            is_admin: true,
            ..Default::default()
        };

        assert_eq!(person, expected_person);
    }
}
