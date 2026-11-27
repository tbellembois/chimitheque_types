#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::producer::Producer;
    use crate::producerref::*;
    use crate::test_helpers::{
        assert_serde_rejects_missing, assert_serde_round_trip, assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "match_exact_search": true,
            "producer_ref_id": 1,
            "producer_ref_label": "REF-1",
            "producer": {"match_exact_search": false, "producer_id": 2, "producer_label": "Acme"}
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<ProducerRef>(&sample());
        assert_eq!(value.producer.producer_id, Some(2));
    }

    #[test]
    fn test_serde_defaults() {
        let value: ProducerRef = serde_json::from_value(json!({
            "producer_ref_label": "REF-1",
            "producer": {"producer_label": "Acme"}
        }))
        .unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.producer_ref_id, None);
        assert!(!value.producer.match_exact_search);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<ProducerRef>(&sample(), &["producer_ref_label", "producer"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        // The label and the label of a new producer (without id) are cleaned.
        let mut value = ProducerRef {
            producer_ref_label: "  REF   1 ".to_string(),
            producer: Producer {
                producer_label: "  Acme   Corp ".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.producer_ref_label, "REF 1");
        assert_eq!(value.producer.producer_label, "Acme Corp");

        // An existing producer is not validated.
        let mut value = ProducerRef {
            producer_ref_label: "REF".to_string(),
            producer: Producer {
                producer_id: Some(3),
                ..Default::default()
            },
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.producer.producer_label, "");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        let mut value = ProducerRef {
            producer_ref_label: "   ".to_string(),
            producer: Producer {
                producer_label: "Acme".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        let error = assert_validation_error(value.sanitize_and_validate(), "producer_ref_label");
        assert!(matches!(error, ValidationError::Empty { .. }));

        // A new producer with an empty label is rejected.
        let mut value = ProducerRef {
            producer_ref_label: "REF".to_string(),
            ..Default::default()
        };
        let error = assert_validation_error(value.sanitize_and_validate(), "producer_label");
        assert!(matches!(error, ValidationError::Empty { .. }));
    }
}
