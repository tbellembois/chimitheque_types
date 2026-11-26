#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::precautionarystatement::*;
    use chimitheque_traits::searchable::Searchable;

    #[test]
    fn test_precautionary_statement_searchable() {
        let precautionary_statement = PrecautionaryStatement {
            match_exact_search: true,
            precautionary_statement_id: Some(1),
            precautionary_statement_label: "test".to_string(),
            ..Default::default()
        };

        let mut created_precautionary_statement = precautionary_statement.create();
        assert!(!created_precautionary_statement.match_exact_search);
        assert_eq!(
            created_precautionary_statement.precautionary_statement_id,
            None
        );
        assert_eq!(
            created_precautionary_statement.precautionary_statement_label,
            String::new()
        );

        created_precautionary_statement.set_exact_search(true);
        assert!(created_precautionary_statement.get_exact_search());

        assert_eq!(
            created_precautionary_statement.get_table_name(),
            "precautionary_statement"
        );
        assert_eq!(
            created_precautionary_statement.get_id_field_name(),
            "precautionary_statement_id"
        );
        assert_eq!(
            created_precautionary_statement.get_text_field_name(),
            "precautionary_statement_reference"
        );

        created_precautionary_statement.set_id_field(2);
        assert_eq!(created_precautionary_statement.get_id(), Some(2));

        created_precautionary_statement.set_text_field("test2");
        assert_eq!(created_precautionary_statement.get_text(), "test2");
    }

    #[test]
    fn test_create_defaults() {
        let precautionary_statement = PrecautionaryStatement::default();
        let created = precautionary_statement.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.precautionary_statement_id, None);
        assert_eq!(created.precautionary_statement_label, String::new());
        assert_eq!(created.precautionary_statement_reference, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut precautionary_statement = PrecautionaryStatement::default();
        precautionary_statement.set_exact_search(true);
        assert!(precautionary_statement.get_exact_search());
        precautionary_statement.set_exact_search(false);
        assert!(!precautionary_statement.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let precautionary_statement = PrecautionaryStatement::default();
        assert_eq!(
            precautionary_statement.get_table_name(),
            "precautionary_statement"
        );
    }

    #[test]
    fn test_get_id_field_name() {
        let precautionary_statement = PrecautionaryStatement::default();
        assert_eq!(
            precautionary_statement.get_id_field_name(),
            "precautionary_statement_id"
        );
    }

    #[test]
    fn test_set_and_get_id() {
        let mut precautionary_statement = PrecautionaryStatement::default();
        precautionary_statement.set_id_field(42);
        assert_eq!(precautionary_statement.get_id(), Some(42));
        precautionary_statement.set_id_field(0);
        assert_eq!(precautionary_statement.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut precautionary_statement = PrecautionaryStatement::default();
        precautionary_statement.set_text_field("P210");
        assert_eq!(precautionary_statement.get_text(), "P210");
        precautionary_statement.set_text_field("");
        assert_eq!(precautionary_statement.get_text(), "");
    }

    #[test]
    fn test_match_exact_search_serde_defaults() -> Result<(), serde_json::Error> {
        let mut payload = serde_json::json!({
            "precautionary_statement_label": "Keep away from heat",
            "precautionary_statement_reference": "P210"
        });
        let value: PrecautionaryStatement = serde_json::from_value(payload.clone())?;
        assert!(!value.match_exact_search);
        assert_eq!(
            serde_json::to_value(&value)?.get("match_exact_search"),
            Some(&serde_json::json!(false))
        );

        for expected in [true, false] {
            payload["match_exact_search"] = serde_json::json!(expected);
            let value: PrecautionaryStatement = serde_json::from_value(payload.clone())?;
            assert_eq!(value.match_exact_search, expected);
            assert_eq!(
                serde_json::to_value(&value)?.get("match_exact_search"),
                Some(&serde_json::json!(expected))
            );
        }

        for required in [
            "precautionary_statement_label",
            "precautionary_statement_reference",
        ] {
            let mut missing_required = payload.clone();
            if let Some(object) = missing_required.as_object_mut() {
                object.remove(required);
            }
            assert!(serde_json::from_value::<PrecautionaryStatement>(missing_required).is_err());
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
        let mut value = PrecautionaryStatement {
            precautionary_statement_label: " Wear  gloves ".to_string(),
            precautionary_statement_reference: " P280 ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.precautionary_statement_label, "Wear gloves");
        assert_eq!(value.precautionary_statement_reference, "P280");

        let mut value = PrecautionaryStatement {
            precautionary_statement_reference: "P280".to_string(),
            ..Default::default()
        };
        assert_empty(
            value.sanitize_and_validate(),
            "precautionary_statement_label",
        );

        let mut value = PrecautionaryStatement {
            precautionary_statement_label: "label".to_string(),
            ..Default::default()
        };
        assert_empty(
            value.sanitize_and_validate(),
            "precautionary_statement_reference",
        );
    }

    #[test]
    fn test_sanitize_and_validate_reference_format() {
        for reference in ["P280", "P301+P317", "P301+P330+P331[+P310]"] {
            let mut value = PrecautionaryStatement {
                precautionary_statement_label: "label".to_string(),
                precautionary_statement_reference: reference.to_string(),
                ..Default::default()
            };
            value.sanitize_and_validate().unwrap();
        }

        for reference in ["H350", "P28", "P280+"] {
            let mut value = PrecautionaryStatement {
                precautionary_statement_label: "label".to_string(),
                precautionary_statement_reference: reference.to_string(),
                ..Default::default()
            };
            assert!(
                matches!(
                    value.sanitize_and_validate().unwrap_err(),
                    crate::error::ValidationError::InvalidFormat {
                        field: "precautionary_statement_reference",
                        ..
                    }
                ),
                "{reference}"
            );
        }
    }

    #[test]
    fn test_is_precautionary_reference() {
        assert!(is_precautionary_reference("P301+P330+P331[+P310]"));
        assert!(!is_precautionary_reference("P301+"));
    }
}
