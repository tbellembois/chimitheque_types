#[cfg(test)]
mod tests {
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
}
