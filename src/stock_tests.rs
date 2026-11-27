#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::test_helpers::{assert_serde_rejects_missing, assert_value_round_trip};
    use crate::{stock::Stock, unit::Unit};

    fn sample() -> Stock {
        Stock {
            quantity: 2.5,
            unit: Some(Unit {
                unit_id: Some(1),
                unit_label: "L".to_string(),
                unit_multiplier: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let json = assert_value_round_trip(&sample());
        assert_eq!(json["quantity"], 2.5);
        assert_eq!(json["unit"]["unit_label"], "L");
    }

    #[test]
    fn test_serde_optional_fields() {
        let mut json = assert_value_round_trip(&sample());
        json.as_object_mut().unwrap().remove("unit");
        let value: Stock = serde_json::from_value(json).unwrap();
        assert!(value.unit.is_none());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        let json = assert_value_round_trip(&sample());
        assert_serde_rejects_missing::<Stock>(&json, &["store_location", "product", "quantity"]);
    }
}
