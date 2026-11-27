#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::validation::*;

    fn expected(result: Result<(), ValidationError>) -> &'static str {
        match result.unwrap_err() {
            ValidationError::OutOfRange { expected, .. } => expected,
            error => panic!("unexpected error {error:?}"),
        }
    }

    #[test]
    fn test_number_checks() {
        for value in [None, Some(0.0), Some(-1.5), Some(2.0)] {
            assert!(check_finite_f64(value, "f").is_ok());
        }
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                expected(check_finite_f64(Some(value), "f")),
                "a finite number"
            );
            assert_eq!(
                expected(check_positive_f64(Some(value), "f")),
                "a finite number"
            );
            assert_eq!(
                expected(check_non_negative_f64(Some(value), "f")),
                "a finite number"
            );
        }

        assert!(check_positive_f64(None, "f").is_ok());
        assert!(check_positive_f64(Some(0.01), "f").is_ok());
        assert_eq!(
            expected(check_positive_f64(Some(0.0), "f")),
            "greater than 0"
        );
        assert_eq!(
            expected(check_positive_f64(Some(-1.0), "f")),
            "greater than 0"
        );

        assert!(check_non_negative_f64(None, "f").is_ok());
        assert!(check_non_negative_f64(Some(0.0), "f").is_ok());
        assert_eq!(
            expected(check_non_negative_f64(Some(-0.1), "f")),
            "0 or more"
        );
    }

    #[test]
    fn test_count_checks() {
        assert!(check_positive_i64(None, "c").is_ok());
        assert!(check_positive_i64(Some(1), "c").is_ok());
        assert_eq!(expected(check_positive_i64(Some(0), "c")), "greater than 0");
        assert_eq!(
            expected(check_positive_i64(Some(-3), "c")),
            "greater than 0"
        );

        assert!(check_non_zero_u64(None, "c").is_ok());
        assert!(check_non_zero_u64(Some(1), "c").is_ok());
        assert_eq!(expected(check_non_zero_u64(Some(0), "c")), "not 0");
    }

    #[test]
    fn test_out_of_range_message() {
        let error = check_positive_f64(Some(0.0), "product_molecular_weight").unwrap_err();
        assert_eq!(error.field(), "product_molecular_weight");
        assert_eq!(
            error.to_string(),
            "product_molecular_weight: must be greater than 0"
        );
    }

    #[test]
    fn test_is_cmr_classification() {
        assert!(is_cmr_classification("C1A M2 R1B"));
        assert!(is_cmr_classification("lact"));
        assert!(!is_cmr_classification(""));
        assert!(!is_cmr_classification("C1A C9"));
    }
}
