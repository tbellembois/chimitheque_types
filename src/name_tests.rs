#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::name::*;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "name_id": 1, "name_label": "ETHYL ALCOHOL"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<Name>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: Name = serde_json::from_value(json!({"name_label": "ETHYL ALCOHOL"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.name_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Name>(&sample(), &["name_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = Name {
            name_label: "  ethyl   alcohol ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.name_label, "ETHYL ALCOHOL");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "  \n\t "] {
            let mut value = Name {
                name_label: empty.to_string(),
                ..Default::default()
            };
            let error = assert_validation_error(value.sanitize_and_validate(), "name_label");
            assert!(matches!(error, ValidationError::Empty { .. }), "{error:?}");
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<Name>("name", "name_id", "name_label");
    }
}
