#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::linearformula::*;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "linear_formula_id": 1, "linear_formula_label": "CH3CH2OH"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<LinearFormula>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: LinearFormula =
            serde_json::from_value(json!({"linear_formula_label": "CH3CH2OH"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.linear_formula_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<LinearFormula>(&sample(), &["linear_formula_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        // The label is cleaned but kept as entered.
        let mut value = LinearFormula {
            linear_formula_label: "  CH3CH2OH ".to_string(),
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.linear_formula_label, "CH3CH2OH");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for invalid in ["", " "] {
            let mut value = LinearFormula {
                linear_formula_label: invalid.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "linear_formula_label");
            assert!(
                matches!(error, ValidationError::Empty { .. }),
                "{invalid}: {error:?}"
            );
        }
        for invalid in ["X123X", "(CH3"] {
            let mut value = LinearFormula {
                linear_formula_label: invalid.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "linear_formula_label");
            assert!(
                matches!(error, ValidationError::InvalidFormula { .. }),
                "{invalid}: {error:?}"
            );
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<LinearFormula>(
            "linear_formula",
            "linear_formula_id",
            "linear_formula_label",
        );
    }
}
