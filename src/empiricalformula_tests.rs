#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::empiricalformula::*;
    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_searchable, assert_serde_rejects_missing, assert_serde_round_trip,
        assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({"match_exact_search": true, "empirical_formula_id": 1, "empirical_formula_label": "C2H6O"})
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        assert_serde_round_trip::<EmpiricalFormula>(&sample());
    }

    #[test]
    fn test_serde_defaults() {
        let value: EmpiricalFormula =
            serde_json::from_value(json!({"empirical_formula_label": "C2H6O"})).unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.empirical_formula_id, None);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<EmpiricalFormula>(&sample(), &["empirical_formula_label"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        // The label is cleaned and stored as its sorted empirical formula.
        for (label, expected) in [("  H2O ", "H2O"), ("CH3CH2OH", "C2H6O")] {
            let mut value = EmpiricalFormula {
                empirical_formula_label: label.to_string(),
                ..Default::default()
            };
            value.sanitize_and_validate().unwrap();
            assert_eq!(value.empirical_formula_label, expected);
        }
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        for empty in ["", " "] {
            let mut value = EmpiricalFormula {
                empirical_formula_label: empty.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "empirical_formula_label");
            assert!(matches!(error, ValidationError::Empty { .. }), "{error:?}");
        }
        // Formulas without any atom are parsed into an empty formula.
        for no_atom in ["()", ".", "2", "x"] {
            let mut value = EmpiricalFormula {
                empirical_formula_label: no_atom.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "empirical_formula_label");
            assert!(
                matches!(error, ValidationError::Empty { .. }),
                "{no_atom}: {error:?}"
            );
        }
        for invalid in ["X123X", "(CH3"] {
            let mut value = EmpiricalFormula {
                empirical_formula_label: invalid.to_string(),
                ..Default::default()
            };
            let error =
                assert_validation_error(value.sanitize_and_validate(), "empirical_formula_label");
            assert!(
                matches!(error, ValidationError::InvalidFormula { .. }),
                "{invalid}: {error:?}"
            );
        }
    }

    // Searchable.

    #[test]
    fn test_searchable() {
        assert_searchable::<EmpiricalFormula>(
            "empirical_formula",
            "empirical_formula_id",
            "empirical_formula_label",
        );
    }
}
