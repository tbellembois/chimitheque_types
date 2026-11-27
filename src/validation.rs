//! Helpers shared by the `sanitize_and_validate` methods.

use crate::error::ValidationError;
use chimitheque_defines::CMR_CODES;

/// Rejects a set number that is not finite (NaN or infinite).
pub(crate) fn check_finite_f64(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    match value {
        Some(number) if !number.is_finite() => Err(ValidationError::OutOfRange {
            field,
            expected: "a finite number",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set number that is not finite or not greater than 0.
pub(crate) fn check_positive_f64(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    check_finite_f64(value, field)?;
    match value {
        Some(number) if number <= 0.0 => Err(ValidationError::OutOfRange {
            field,
            expected: "greater than 0",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set number that is not finite or is negative.
pub(crate) fn check_non_negative_f64(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    check_finite_f64(value, field)?;
    match value {
        Some(number) if number < 0.0 => Err(ValidationError::OutOfRange {
            field,
            expected: "0 or more",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set count that is not greater than 0.
pub(crate) fn check_positive_i64(
    value: Option<i64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    match value {
        Some(count) if count <= 0 => Err(ValidationError::OutOfRange {
            field,
            expected: "greater than 0",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set unsigned count equal to 0.
pub(crate) fn check_non_zero_u64(
    value: Option<u64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    match value {
        Some(0) => Err(ValidationError::OutOfRange {
            field,
            expected: "not 0",
        }),
        _ => Ok(()),
    }
}

/// Returns true for space-separated CMR codes, e.g. `C1A M2`, ignoring case.
pub(crate) fn is_cmr_classification(classification: &str) -> bool {
    let mut codes = classification.split_whitespace().peekable();
    codes.peek().is_some()
        && codes.all(|code| {
            CMR_CODES
                .iter()
                .any(|known| known.eq_ignore_ascii_case(code))
        })
}

#[cfg(test)]
#[path = "validation_tests.rs"]
mod validation_tests;
