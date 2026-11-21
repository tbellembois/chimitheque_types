#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::unit::*;

    fn unit(label: &str, multiplier: f64) -> Unit {
        Unit {
            unit_label: label.to_string(),
            unit_multiplier: multiplier,
            ..Default::default()
        }
    }

    #[test]
    fn test_sanitize_and_validate_keeps_label_case() {
        let mut value = unit("  mL ", 0.001);
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.unit_label, "mL");
    }

    #[test]
    fn test_sanitize_and_validate_rejects_empty_label() {
        let error = unit("  ", 1.0).sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::Empty {
                field: "unit_label"
            }
        ));
    }

    #[test]
    fn test_sanitize_and_validate_rejects_invalid_multiplier() {
        for multiplier in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let error = unit("g", multiplier).sanitize_and_validate().unwrap_err();
            assert!(
                matches!(
                    error,
                    ValidationError::OutOfRange {
                        field: "unit_multiplier",
                        ..
                    }
                ),
                "{multiplier}: {error:?}"
            );
        }
    }
}
