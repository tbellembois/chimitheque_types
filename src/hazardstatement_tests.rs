#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::hazardstatement::*;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "hazard_statement_id": 1, "hazard_statement_label": "May cause cancer", "hazard_statement_reference": "H350", "hazard_statement_cmr": "C1A"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<HazardStatement>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: HazardStatement = serde_json::from_value(json!({"hazard_statement_label": "May cause cancer", "hazard_statement_reference": "H350"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.hazard_statement_id, None);
        assert_eq!(value.hazard_statement_cmr, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<HazardStatement>(
            &sample(),
            &["hazard_statement_label", "hazard_statement_reference"],
        );
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = HazardStatement {
            hazard_statement_label: "  May   cause cancer ".to_string(),
            hazard_statement_reference: " H350 ".to_string(),
            hazard_statement_cmr: Some(" C1A   M2 ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.hazard_statement_label, "May cause cancer");
        assert_eq!(value.hazard_statement_reference, "H350");
        assert_eq!(value.hazard_statement_cmr.as_deref(), Some("C1A M2"));

        // An empty CMR classification becomes None.
        value.hazard_statement_cmr = Some("  ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.hazard_statement_cmr, None);
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        let valid = || HazardStatement {
            hazard_statement_label: "May cause cancer".to_string(),
            hazard_statement_reference: "H350".to_string(),
            ..Default::default()
        };

        let mut value = valid();
        value.hazard_statement_label = " ".to_string();
        let error =
            assert_validation_error(value.sanitize_and_validate(), "hazard_statement_label");
        assert!(matches!(error, ValidationError::Empty { .. }));

        let mut value = valid();
        value.hazard_statement_reference = String::new();
        let error =
            assert_validation_error(value.sanitize_and_validate(), "hazard_statement_reference");
        assert!(matches!(error, ValidationError::Empty { .. }));

        let mut value = valid();
        value.hazard_statement_reference = "P280".to_string();
        let error =
            assert_validation_error(value.sanitize_and_validate(), "hazard_statement_reference");
        assert!(matches!(error, ValidationError::InvalidFormat { .. }));

        let mut value = valid();
        value.hazard_statement_cmr = Some("C1A X9".to_string());
        let error = assert_validation_error(value.sanitize_and_validate(), "hazard_statement_cmr");
        assert!(matches!(error, ValidationError::InvalidFormat { .. }));
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<HazardStatement>(
            "hazard_statement",
            "hazard_statement_id",
            "hazard_statement_reference",
        );
    }

    // Other functions.

    #[test]
    fn test_is_hazard_reference() {
        assert!(is_hazard_reference("H361fd"));
        assert!(is_hazard_reference("EUH001+H300"));
        assert!(!is_hazard_reference("H35"));
        assert!(!is_hazard_reference("H350 "));
    }
}
