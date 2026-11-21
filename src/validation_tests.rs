#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use chimitheque_utils::string::Transform;

    use crate::error::ValidationError;
    use crate::validation::*;

    fn expected(result: Result<(), ValidationError>) -> &'static str {
        match result.unwrap_err() {
            ValidationError::OutOfRange { expected, .. } => expected,
            error => panic!("unexpected error {error:?}"),
        }
    }

    #[test]
    fn test_require_text() {
        let mut value = "  a   b ".to_string();
        require_text(&mut value, "field", Transform::None).unwrap();
        assert_eq!(value, "a b");

        let mut value = "ab".to_string();
        require_text(&mut value, "field", Transform::Uppercase).unwrap();
        assert_eq!(value, "AB");

        for empty in ["", "   ", "\n\t"] {
            let mut value = empty.to_string();
            let error = require_text(&mut value, "field", Transform::None).unwrap_err();
            assert!(matches!(error, ValidationError::Empty { field: "field" }));
        }
    }

    #[test]
    fn test_clean_optional_text() {
        let mut value = Some("  a  \n b ".to_string());
        clean_optional_text(&mut value);
        assert_eq!(value.as_deref(), Some("a b"));

        for empty in [Some(String::new()), Some("  \n ".to_string()), None] {
            let mut value = empty;
            clean_optional_text(&mut value);
            assert_eq!(value, None);
        }
    }

    #[test]
    fn test_trim_optional_text_keeps_line_breaks() {
        let mut value = Some("  line 1\n\n  line 2  ".to_string());
        trim_optional_text(&mut value);
        assert_eq!(value.as_deref(), Some("line 1\n\n  line 2"));

        let mut value = Some(" \n ".to_string());
        trim_optional_text(&mut value);
        assert_eq!(value, None);
    }

    #[test]
    fn test_number_checks() {
        for value in [None, Some(0.0), Some(-1.5), Some(2.0)] {
            assert!(check_finite(value, "f").is_ok());
        }
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(expected(check_finite(Some(value), "f")), "a finite number");
            assert_eq!(
                expected(check_positive(Some(value), "f")),
                "a finite number"
            );
            assert_eq!(
                expected(check_non_negative(Some(value), "f")),
                "a finite number"
            );
        }

        assert!(check_positive(None, "f").is_ok());
        assert!(check_positive(Some(0.01), "f").is_ok());
        assert_eq!(expected(check_positive(Some(0.0), "f")), "greater than 0");
        assert_eq!(expected(check_positive(Some(-1.0), "f")), "greater than 0");

        assert!(check_non_negative(None, "f").is_ok());
        assert!(check_non_negative(Some(0.0), "f").is_ok());
        assert_eq!(expected(check_non_negative(Some(-0.1), "f")), "0 or more");
    }

    #[test]
    fn test_count_checks() {
        assert!(check_positive_count(None, "c").is_ok());
        assert!(check_positive_count(Some(1), "c").is_ok());
        assert_eq!(
            expected(check_positive_count(Some(0), "c")),
            "greater than 0"
        );
        assert_eq!(
            expected(check_positive_count(Some(-3), "c")),
            "greater than 0"
        );

        assert!(check_non_zero_count(None, "c").is_ok());
        assert!(check_non_zero_count(Some(1), "c").is_ok());
        assert_eq!(
            expected(check_non_zero_count(Some(0), "c")),
            "greater than 0"
        );
    }

    #[test]
    fn test_out_of_range_message() {
        let error = check_positive(Some(0.0), "product_molecular_weight").unwrap_err();
        assert_eq!(error.field(), "product_molecular_weight");
        assert_eq!(
            error.to_string(),
            "product_molecular_weight: must be greater than 0"
        );
    }

    #[test]
    fn test_reference_color_and_cmr_formats() {
        assert!(is_hazard_reference("H361fd"));
        assert!(is_hazard_reference("EUH001+H300"));
        assert!(!is_hazard_reference("H35"));
        assert!(!is_hazard_reference("H350 "));

        assert!(is_precautionary_reference("P301+P330+P331[+P310]"));
        assert!(!is_precautionary_reference("P301+"));

        assert!(is_html_color("#A1b2C3"));
        assert!(!is_html_color("#A1b2C"));

        assert!(is_cmr_classification("C1A M2 R1B"));
        assert!(is_cmr_classification("lact"));
        assert!(!is_cmr_classification(""));
        assert!(!is_cmr_classification("C1A C9"));
    }

    #[test]
    fn test_require_if() {
        assert!(require_if(false, false, "f", "when x is set").is_ok());
        assert!(require_if(true, true, "f", "when x is set").is_ok());
        let error = require_if(true, false, "f", "when x is set").unwrap_err();
        assert_eq!(error.to_string(), "f: required when x is set");
    }
}
