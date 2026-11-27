#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::physicalstate::*;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "physical_state_id": 1, "physical_state_label": "liquid"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<PhysicalState>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: PhysicalState =
            serde_json::from_value(json!({"physical_state_label": "liquid"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.physical_state_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<PhysicalState>(&sample(), &["physical_state_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        let mut value = PhysicalState {
            physical_state_label: "  liquid ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.physical_state_label, "liquid");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", "  \n\t "] {
            let mut value = PhysicalState {
                physical_state_label: empty.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "physical_state_label");
            assert!(matches!(error, ValidationError::Empty { .. }), "{error:?}");
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<PhysicalState>(
            "physical_state",
            "physical_state_id",
            "physical_state_label",
        );
    }
}
