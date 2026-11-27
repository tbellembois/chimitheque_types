#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::cenumber::*;
    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "ce_number_id": 1, "ce_number_label": "214-480-6"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<CeNumber>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: CeNumber =
            serde_json::from_value(json!({"ce_number_label": "214-480-6"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.ce_number_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<CeNumber>(&sample(), &["ce_number_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = CeNumber {
            ce_number_label: "  214-480-6  ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.ce_number_label, "214-480-6");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for invalid in ["", " "] {
            let mut value = CeNumber {
                ce_number_label: invalid.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "ce_number_label");
            assert!(
                matches!(error, ValidationError::Empty { .. }),
                "{invalid}: {error:?}"
            );
        }
        for invalid in ["12345", "abcdef", "214-480-5"] {
            let mut value = CeNumber {
                ce_number_label: invalid.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "ce_number_label");
            assert!(
                matches!(error, ValidationError::InvalidCeNumber(_)),
                "{invalid}: {error:?}"
            );
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<CeNumber>("ce_number", "ce_number_id", "ce_number_label");
    }
}
