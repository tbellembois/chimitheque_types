#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::producer::*;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "producer_id": 1, "producer_label": "Acme Corp"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Producer>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: Producer =
            serde_json::from_value(json!({"producer_label": "Acme Corp"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.producer_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Producer>(&sample(), &["producer_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = Producer {
            producer_label: "  Acme   Corp ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.producer_label, "Acme Corp");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "  \n\t "] {
            let mut value = Producer {
                producer_label: empty.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "producer_label");
            assert!(matches!(error, ValidationError::Empty { .. }), "{error:?}");
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<Producer>("producer", "producer_id", "producer_label");
    }
}
