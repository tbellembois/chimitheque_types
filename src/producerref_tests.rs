#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::producer::Producer;
    use crate::producerref::*;

    #[test]
    fn test_match_exact_search_serde_defaults() -> Result<(), serde_json::Error> {
        let mut payload = serde_json::json!({
            "producer_ref_label": "P-001",
            "producer": {"producer_label": "Producer"}
        });
        let value: ProducerRef = serde_json::from_value(payload.clone())?;
        assert!(!value.match_exact_search);
        assert_eq!(
            serde_json::to_value(&value)?.get("match_exact_search"),
            Some(&serde_json::json!(false))
        );
        assert!(!value.producer.match_exact_search);

        for expected in [true, false] {
            payload["match_exact_search"] = serde_json::json!(expected);
            let value: ProducerRef = serde_json::from_value(payload.clone())?;
            assert_eq!(value.match_exact_search, expected);
            assert_eq!(
                serde_json::to_value(&value)?.get("match_exact_search"),
                Some(&serde_json::json!(expected))
            );
        }

        for required in ["producer_ref_label", "producer"] {
            let mut missing_required = payload.clone();
            if let Some(object) = missing_required.as_object_mut() {
                object.remove(required);
            }
            assert!(serde_json::from_value::<ProducerRef>(missing_required).is_err());
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

        let mut value = ProducerRef {
            producer_ref_label: "   ".to_string(),
            ..Default::default()
        };
        assert_empty(value.sanitize_and_validate(), "producer_ref_label");

        // A new producer with an empty label is rejected.
        let mut value = ProducerRef {
            producer_ref_label: "REF".to_string(),
            ..Default::default()
        };
        assert_empty(value.sanitize_and_validate(), "producer_label");

        // An existing producer is left unchanged.
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
}
