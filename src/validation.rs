//! Helpers shared by the `sanitize_and_validate` methods.

use chimitheque_defines::{
    CMR_CODES, HAZARD_REFERENCE_RE, HTML_COLOR_RE, PRECAUTIONARY_REFERENCE_RE,
};
use chimitheque_utils::string::{Transform, clean};

use crate::{error::ValidationError, unit::Unit, unittype::UnitType};

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

/// Rejects a set text value for which `is_valid` returns false.
pub(crate) fn check_format(
    value: Option<&str>,
    field: &'static str,
    is_valid: impl Fn(&str) -> bool,
    expected: &'static str,
) -> Result<(), ValidationError> {
    match value {
        Some(text) if !is_valid(text) => Err(ValidationError::InvalidFormat { field, expected }),
        _ => Ok(()),
    }
}

/// Returns true for a GHS hazard statement reference, e.g. `H350` or `H300+H310`.
pub(crate) fn is_hazard_reference(reference: &str) -> bool {
    HAZARD_REFERENCE_RE.is_match(reference)
}

/// Returns true for a GHS precautionary statement reference, e.g. `P280` or
/// `P301+P317`.
pub(crate) fn is_precautionary_reference(reference: &str) -> bool {
    PRECAUTIONARY_REFERENCE_RE.is_match(reference)
}

/// Returns true for an HTML color like `#ff0000`.
pub(crate) fn is_html_color(color: &str) -> bool {
    HTML_COLOR_RE.is_match(color)
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

/// Requires `field` when the field it depends on is set.
pub(crate) fn require_if(
    required: bool,
    present: bool,
    field: &'static str,
    reason: &'static str,
) -> Result<(), ValidationError> {
    if required && !present {
        return Err(ValidationError::Required { field, reason });
    }
    Ok(())
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
