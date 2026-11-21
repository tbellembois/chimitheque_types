#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::hazardstatement::*;
    use chimitheque_traits::searchable::Searchable;

    #[test]
    fn test_hazard_statement_searchable() {
        let hazard_statement = HazardStatement {
            match_exact_search: true,
            hazard_statement_id: Some(1),
            hazard_statement_label: "test".to_string(),
            ..Default::default()
        };

        let mut created_hazard_statement = hazard_statement.create();
        assert!(!created_hazard_statement.match_exact_search);
        assert_eq!(created_hazard_statement.hazard_statement_id, None);
        assert_eq!(
            created_hazard_statement.hazard_statement_label,
            String::new()
        );

        created_hazard_statement.set_exact_search(true);
        assert!(created_hazard_statement.get_exact_search());

        assert_eq!(
            created_hazard_statement.get_table_name(),
            "hazard_statement"
        );
        assert_eq!(
            created_hazard_statement.get_id_field_name(),
            "hazard_statement_id"
        );
        assert_eq!(
            created_hazard_statement.get_text_field_name(),
            "hazard_statement_reference"
        );

        created_hazard_statement.set_id_field(2);
        assert_eq!(created_hazard_statement.get_id(), Some(2));

        created_hazard_statement.set_text_field("test2");
        assert_eq!(created_hazard_statement.get_text(), "test2");
    }

    #[test]
    fn test_create_defaults() {
        let hazard_statement = HazardStatement::default();
        let created = hazard_statement.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.hazard_statement_id, None);
        assert_eq!(created.hazard_statement_label, String::new());
        assert_eq!(created.hazard_statement_reference, String::new());
        assert_eq!(created.hazard_statement_cmr, None);
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut hazard_statement = HazardStatement::default();
        hazard_statement.set_exact_search(true);
        assert!(hazard_statement.get_exact_search());
        hazard_statement.set_exact_search(false);
        assert!(!hazard_statement.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let hazard_statement = HazardStatement::default();
        assert_eq!(hazard_statement.get_table_name(), "hazard_statement");
    }

    #[test]
    fn test_get_id_field_name() {
        let hazard_statement = HazardStatement::default();
        assert_eq!(hazard_statement.get_id_field_name(), "hazard_statement_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut hazard_statement = HazardStatement::default();
        hazard_statement.set_id_field(42);
        assert_eq!(hazard_statement.get_id(), Some(42));
        hazard_statement.set_id_field(0);
        assert_eq!(hazard_statement.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut hazard_statement = HazardStatement::default();
        hazard_statement.set_text_field("H225");
        assert_eq!(hazard_statement.get_text(), "H225");
        hazard_statement.set_text_field("");
        assert_eq!(hazard_statement.get_text(), "");
    }

    #[test]
    fn test_match_exact_search_serde_defaults() -> Result<(), serde_json::Error> {
        let mut payload = serde_json::json!({
            "hazard_statement_label": "Highly flammable liquid and vapour",
            "hazard_statement_reference": "H225"
        });
        let value: HazardStatement = serde_json::from_value(payload.clone())?;
        assert!(!value.match_exact_search);
        assert_eq!(
            serde_json::to_value(&value)?.get("match_exact_search"),
            Some(&serde_json::json!(false))
        );

        for expected in [true, false] {
            payload["match_exact_search"] = serde_json::json!(expected);
            let value: HazardStatement = serde_json::from_value(payload.clone())?;
            assert_eq!(value.match_exact_search, expected);
            assert_eq!(
                serde_json::to_value(&value)?.get("match_exact_search"),
                Some(&serde_json::json!(expected))
            );
        }

        for required in ["hazard_statement_label", "hazard_statement_reference"] {
            let mut missing_required = payload.clone();
            if let Some(object) = missing_required.as_object_mut() {
                object.remove(required);
            }
            assert!(serde_json::from_value::<HazardStatement>(missing_required).is_err());
        }
        Ok(())
    }

    fn assert_empty(result: Result<(), crate::error::ValidationError>, expected_field: &str) {
        let error = result.unwrap_err();
        assert!(
            matches!(error, crate::error::ValidationError::Empty { field } if field == expected_field),
            "{error:?}"
        );
    }

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = HazardStatement {
            hazard_statement_label: " May  cause cancer ".to_string(),
            hazard_statement_reference: " H350 ".to_string(),
            hazard_statement_cmr: Some("  ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.hazard_statement_label, "May cause cancer");
        assert_eq!(value.hazard_statement_reference, "H350");
        assert_eq!(value.hazard_statement_cmr, None);

        let mut value = HazardStatement {
            hazard_statement_reference: "H350".to_string(),
            ..Default::default()
        };
        assert_empty(value.sanitize_and_validate(), "hazard_statement_label");

        let mut value = HazardStatement {
            hazard_statement_label: "label".to_string(),
            ..Default::default()
        };
        assert_empty(value.sanitize_and_validate(), "hazard_statement_reference");
    }

    #[test]
    fn test_sanitize_and_validate_reference_and_cmr_format() {
        for reference in ["H350", "H360Fd", "H300+H310", "EUH014"] {
            let mut value = HazardStatement {
                hazard_statement_label: "label".to_string(),
                hazard_statement_reference: reference.to_string(),
                hazard_statement_cmr: Some("c1a m2".to_string()),
                ..Default::default()
            };
            value.sanitize_and_validate().unwrap();
        }

        let mut value = HazardStatement {
            hazard_statement_label: "label".to_string(),
            hazard_statement_reference: "P280".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            value.sanitize_and_validate().unwrap_err(),
            crate::error::ValidationError::InvalidFormat {
                field: "hazard_statement_reference",
                ..
            }
        ));

        let mut value = HazardStatement {
            hazard_statement_label: "label".to_string(),
            hazard_statement_reference: "H350".to_string(),
            hazard_statement_cmr: Some("C1A X9".to_string()),
            ..Default::default()
        };
        assert!(matches!(
            value.sanitize_and_validate().unwrap_err(),
            crate::error::ValidationError::InvalidFormat {
                field: "hazard_statement_cmr",
                ..
            }
        ));
    }
}
