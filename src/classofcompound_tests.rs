#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::classofcompound::*;
    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "class_of_compound_id": 1, "class_of_compound_label": "alcohols"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<ClassOfCompound>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: ClassOfCompound =
            serde_json::from_value(json!({"class_of_compound_label": "alcohols"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.class_of_compound_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<ClassOfCompound>(&sample(), &["class_of_compound_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = ClassOfCompound {
            class_of_compound_label: "  alcohols  ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.class_of_compound_label, "alcohols");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "  \n\t "] {
            let mut value = ClassOfCompound {
                class_of_compound_label: empty.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "class_of_compound_label");
            assert!(matches!(error, ValidationError::Empty { .. }), "{error:?}");
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<ClassOfCompound>(
            "class_of_compound",
            "class_of_compound_id",
            "class_of_compound_label",
        );
    }
}
