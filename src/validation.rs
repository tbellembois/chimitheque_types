//! Helpers shared by the `sanitize_and_validate` methods.

use chimitheque_utils::string::{Transform, clean};

use crate::error::ValidationError;

/// Cleans a required text field and rejects it if empty.
pub(crate) fn require_text(
    value: &mut String,
    field: &'static str,
    transform: Transform,
) -> Result<(), ValidationError> {
    *value = clean(value, transform);
    if value.is_empty() {
        return Err(ValidationError::Empty { field });
    }
    Ok(())
}

/// Cleans an optional single-line text field (trims and collapses inner
/// whitespace). An empty value becomes `None`.
pub(crate) fn clean_optional_text(value: &mut Option<String>) {
    if let Some(text) = value.as_mut() {
        *text = clean(text, Transform::None);
    }
    none_if_empty(value);
}

/// Trims an optional free-text field, keeping inner whitespace and line
/// breaks. An empty value becomes `None`.
pub(crate) fn trim_optional_text(value: &mut Option<String>) {
    if let Some(text) = value.as_mut() {
        let trimmed = text.trim();
        if trimmed.len() != text.len() {
            *text = trimmed.to_string();
        }
    }
    none_if_empty(value);
}

fn none_if_empty(value: &mut Option<String>) {
    if value.as_deref().is_some_and(str::is_empty) {
        *value = None;
    }
}

/// Rejects a set number that is not finite (NaN or infinite).
pub(crate) fn check_finite(value: Option<f64>, field: &'static str) -> Result<(), ValidationError> {
    match value {
        Some(number) if !number.is_finite() => Err(ValidationError::OutOfRange {
            field,
            expected: "a finite number",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set number that is not finite or not greater than 0.
pub(crate) fn check_positive(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    check_finite(value, field)?;
    match value {
        Some(number) if number <= 0.0 => Err(ValidationError::OutOfRange {
            field,
            expected: "greater than 0",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set number that is not finite or is negative.
pub(crate) fn check_non_negative(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    check_finite(value, field)?;
    match value {
        Some(number) if number < 0.0 => Err(ValidationError::OutOfRange {
            field,
            expected: "0 or more",
        }),
        _ => Ok(()),
    }
}

/// Rejects a set count that is not greater than 0.
pub(crate) fn check_positive_count(
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
pub(crate) fn check_non_zero_count(
    value: Option<u64>,
    field: &'static str,
) -> Result<(), ValidationError> {
    match value {
        Some(0) => Err(ValidationError::OutOfRange {
            field,
            expected: "greater than 0",
        }),
        _ => Ok(()),
    }
}

#[cfg(test)]
#[path = "validation_tests.rs"]
mod validation_tests;
