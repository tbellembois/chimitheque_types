#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::precautionarystatement::*;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "precautionary_statement_id": 1, "precautionary_statement_label": "Wear protective gloves", "precautionary_statement_reference": "P280"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<PrecautionaryStatement>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: PrecautionaryStatement = serde_json::from_value(json!({"precautionary_statement_label": "Wear protective gloves", "precautionary_statement_reference": "P280"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.precautionary_statement_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<PrecautionaryStatement>(
            &sample(),
            &[
                "precautionary_statement_label",
                "precautionary_statement_reference",
            ],
        );
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = PrecautionaryStatement {
            precautionary_statement_label: "  Wear   protective gloves ".to_string(),
            precautionary_statement_reference: " P301+P317 ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(
            value.precautionary_statement_label,
            "Wear protective gloves"
        );
        assert_eq!(value.precautionary_statement_reference, "P301+P317");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        let valid = || PrecautionaryStatement {
            precautionary_statement_label: "Wear protective gloves".to_string(),
            precautionary_statement_reference: "P280".to_string(),
            ..Default::default()
        };

        let mut value = valid();
        value.precautionary_statement_label = " ".to_string();
        let error = assert_validation_error(
            value.sanitize_and_validate(),
            "precautionary_statement_label",
        );
        assert!(matches!(error, ValidationError::Empty { .. }));

        let mut value = valid();
        value.precautionary_statement_reference = String::new();
        let error = assert_validation_error(
            value.sanitize_and_validate(),
            "precautionary_statement_reference",
        );
        assert!(matches!(error, ValidationError::Empty { .. }));

        for invalid in ["H350", "P301+", "280"] {
            let mut value = valid();
            value.precautionary_statement_reference = invalid.to_string();
            let error = assert_validation_error(
                value.sanitize_and_validate(),
                "precautionary_statement_reference",
            );
            assert!(
                matches!(error, ValidationError::InvalidFormat { .. }),
                "{invalid}"
            );
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<PrecautionaryStatement>(
            "precautionary_statement",
            "precautionary_statement_id",
            "precautionary_statement_reference",
        );
    }

    // Other functions.

    #[test]
    fn test_is_precautionary_reference() {
        assert!(is_precautionary_reference("P301+P330+P331[+P310]"));
        assert!(!is_precautionary_reference("P301+"));
    }
}
