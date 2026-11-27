#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::category::*;
    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "category_id": 1, "category_label": "organic acids"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Category>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: Category =
            serde_json::from_value(json!({"category_label": "organic acids"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.category_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Category>(&sample(), &["category_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = Category {
            category_label: "  organic   acids ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.category_label, "organic acids");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "  \n\t "] {
            let mut value = Category {
                category_label: empty.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "category_label");
            assert!(matches!(error, ValidationError::Empty { .. }), "{error:?}");
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<Category>("category", "category_id", "category_label");
    }
}
