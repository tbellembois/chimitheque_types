#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::producer::*;
    use chimitheque_traits::searchable::Searchable;

    #[test]
    fn test_producer_searchable() {
        let producer = Producer {
            match_exact_search: true,
            producer_id: Some(1),
            producer_label: "test".to_string(),
        };

        let mut created_producer = producer.create();
        assert!(!created_producer.match_exact_search);
        assert_eq!(created_producer.producer_id, None);
        assert_eq!(created_producer.producer_label, String::new());

        created_producer.set_exact_search(true);
        assert!(created_producer.get_exact_search());

        assert_eq!(created_producer.get_table_name(), "producer");
        assert_eq!(created_producer.get_id_field_name(), "producer_id");
        assert_eq!(created_producer.get_text_field_name(), "producer_label");

        created_producer.set_id_field(2);
        assert_eq!(created_producer.get_id(), Some(2));

        created_producer.set_text_field("test2");
        assert_eq!(created_producer.get_text(), "test2");
    }

    #[test]
    fn test_create_defaults() {
        let producer = Producer::default();
        let created = producer.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.producer_id, None);
        assert_eq!(created.producer_label, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut producer = Producer::default();
        producer.set_exact_search(true);
        assert!(producer.get_exact_search());
        producer.set_exact_search(false);
        assert!(!producer.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let producer = Producer::default();
        assert_eq!(producer.get_table_name(), "producer");
    }

    #[test]
    fn test_get_id_field_name() {
        let producer = Producer::default();
        assert_eq!(producer.get_id_field_name(), "producer_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut producer = Producer::default();
        producer.set_id_field(42);
        assert_eq!(producer.get_id(), Some(42));
        producer.set_id_field(0);
        assert_eq!(producer.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut producer = Producer::default();
        producer.set_text_field("Acme Corp");
        assert_eq!(producer.get_text(), "Acme Corp");
        producer.set_text_field("");
        assert_eq!(producer.get_text(), "");
    }

    #[test]
    fn test_sanitize_and_validate_leading_trailing_spaces() {
        let mut producer = Producer {
            match_exact_search: false,
            producer_id: Some(1),
            producer_label: "  hello world  ".to_string(),
        };
        assert!(producer.sanitize_and_validate().is_ok());
        assert_eq!(producer.producer_label, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_multiple_spaces() {
        let mut producer = Producer {
            match_exact_search: false,
            producer_id: Some(1),
            producer_label: "  hello    world  ".to_string(),
        };
        assert!(producer.sanitize_and_validate().is_ok());
        assert_eq!(producer.producer_label, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_empty_label() {
        let mut producer = Producer {
            match_exact_search: false,
            producer_id: Some(1),
            producer_label: String::default(),
        };
        assert!(producer.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_no_spaces() {
        let mut producer = Producer {
            match_exact_search: false,
            producer_id: Some(1),
            producer_label: "hello world".to_string(),
        };
        assert!(producer.sanitize_and_validate().is_ok());
        assert_eq!(producer.producer_label, "hello world");
    }

    #[test]
    fn test_match_exact_search_serde_defaults() -> Result<(), serde_json::Error> {
        let mut payload = serde_json::json!({"producer_label": "Producer"});
        let value: Producer = serde_json::from_value(payload.clone())?;
        assert!(!value.match_exact_search);
        assert_eq!(
            serde_json::to_value(&value)?.get("match_exact_search"),
            Some(&serde_json::json!(false))
        );

        for expected in [true, false] {
            payload["match_exact_search"] = serde_json::json!(expected);
            let value: Producer = serde_json::from_value(payload.clone())?;
            assert_eq!(value.match_exact_search, expected);
            assert_eq!(
                serde_json::to_value(&value)?.get("match_exact_search"),
                Some(&serde_json::json!(expected))
            );
        }

        let mut missing_required = payload.clone();
        if let Some(object) = missing_required.as_object_mut() {
            object.remove("producer_label");
        }
        assert!(serde_json::from_value::<Producer>(missing_required).is_err());
        Ok(())
    }
}
