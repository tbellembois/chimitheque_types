#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::casnumber::*;
    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "cas_number_id": 1, "cas_number_label": "7732-18-5", "cas_number_cmr": "C1A M2"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<CasNumber>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: CasNumber =
            serde_json::from_value(json!({"cas_number_label": "7732-18-5"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.cas_number_id, None);
        assert_eq!(value.cas_number_cmr, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<CasNumber>(&sample(), &["cas_number_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = CasNumber {
            cas_number_label: "  7732-18-5  ".to_string(),
            cas_number_cmr: Some("  C1A   lact ".to_string()),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.cas_number_label, "7732-18-5");
        assert_eq!(value.cas_number_cmr.as_deref(), Some("C1A lact"));

        // An empty CMR classification becomes None.
        value.cas_number_cmr = Some("  ".to_string());
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.cas_number_cmr, None);
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for invalid in ["", " "] {
            let mut value = CasNumber {
                cas_number_label: invalid.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "cas_number_label");
            assert!(
                matches!(error, ValidationError::Empty { .. }),
                "{invalid}: {error:?}"
            );
        }
        for invalid in ["12345", "abcdef", "7732-18-4"] {
            let mut value = CasNumber {
                cas_number_label: invalid.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "cas_number_label");
            assert!(
                matches!(error, ValidationError::InvalidCasNumber(_)),
                "{invalid}: {error:?}"
            );
        }
        for invalid in ["C1A unknown", "C3"] {
            let mut value = CasNumber {
                cas_number_label: "7732-18-5".to_string(),
                cas_number_cmr: Some(invalid.to_string()),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "cas_number_cmr");
            assert!(
                matches!(error, ValidationError::InvalidFormat { .. }),
                "{invalid}: {error:?}"
            );
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<CasNumber>("cas_number", "cas_number_id", "cas_number_label");
    }
}
