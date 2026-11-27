#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::test_helpers::{
        assert_serde_rejects_missing, assert_serde_round_trip, assert_validation_error,
    };
    use crate::unit::*;
    use crate::unittype::UnitType;
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "unit_id": 1,
            "unit_label": "mL",
            "unit_multiplier": 0.001,
            "unit_type": "quantity",
            "unit": {
                "unit_id": 2,
                "unit_label": "L",
                "unit_multiplier": 1.0,
                "unit_type": "quantity",
                "unit": null
            }
        })
    }

    fn unit(label: &str, multiplier: f64) -> Unit {
        Unit {
            unit_label: label.to_string(),
            unit_multiplier: multiplier,
            ..Default::default()
        }
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<Unit>(&sample());
        assert_eq!(value.unit_type, UnitType::Quantity);
        assert_eq!(value.unit.unwrap().unit_label, "L");
    }

    #[test]
    fn test_serde_defaults() {
        let value: Unit =
            serde_json::from_value(json!({"unit_label": "g", "unit_multiplier": 1.0})).unwrap();
        assert_eq!(value.unit_id, None);
        assert_eq!(value.unit_type, UnitType::default());
        assert!(value.unit.is_none());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Unit>(&sample(), &["unit_label", "unit_multiplier"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        // Unit labels are case-sensitive: only whitespace is cleaned.
        let mut value = unit("  mL ", 0.001);
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.unit_label, "mL");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        let error = assert_validation_error(unit("  ", 1.0).sanitize_and_validate(), "unit_label");
        assert!(matches!(error, ValidationError::Empty { .. }));

        for multiplier in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            let error = assert_validation_error(
                unit("g", multiplier).sanitize_and_validate(),
                "unit_multiplier",
            );
            assert!(
                matches!(error, ValidationError::OutOfRange { .. }),
                "{multiplier}: {error:?}"
            );
        }
    }
}
