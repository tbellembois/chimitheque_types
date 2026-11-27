//! Helpers shared by the `sanitize_and_validate` methods.

use chimitheque_defines::CMR_CODES;
use chimitheque_utils::string::{Transform, clean};

use crate::{error::ValidationError, unit::Unit, unittype::UnitType};

/// Cleans a required text field and rejects it if empty.
pub(crate) fn clean_required_text(
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

/// Cleans an optional text field (trims each line and collapses its inner
/// whitespace, keeping the line breaks). An empty value becomes `None`.
pub(crate) fn clean_optional_text(value: &mut Option<String>) {
    if let Some(text) = value.as_mut() {
        *text = clean(text, Transform::None);
    }
    if value.as_deref().is_some_and(str::is_empty) {
        *value = None;
    }
}

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

/// Rejects a set unit whose type, looked up from its id with `unit_type_of`,
/// is not `expected`. A unit without an id or with an unknown id is rejected too.
pub(crate) fn check_unit_type(
    unit: Option<&Unit>,
    field: &'static str,
    expected: UnitType,
    unit_type_of: &impl Fn(u64) -> Option<UnitType>,
) -> Result<(), ValidationError> {
    let Some(unit) = unit else {
        return Ok(());
    };
    match unit.unit_id.and_then(unit_type_of) {
        Some(unit_type) if unit_type == expected => Ok(()),
        _ => Err(ValidationError::InvalidUnit { field, expected }),
    }
}

#[cfg(test)]
#[path = "validation_tests.rs"]
mod validation_tests;
