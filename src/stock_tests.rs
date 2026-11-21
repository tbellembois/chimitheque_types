#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::{stock::Stock, unit::Unit};

    #[test]
    fn test_serde_round_trip() {
        let stock = Stock {
            quantity: 2.5,
            unit: Some(Unit {
                unit_id: Some(1),
                unit_label: "L".to_string(),
                unit_multiplier: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_value(&stock).unwrap();
        assert_eq!(json["quantity"], 2.5);
        assert_eq!(json["unit"]["unit_label"], "L");

        let parsed: Stock = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(serde_json::to_value(&parsed).unwrap(), json);
    }

    #[test]
    fn test_serde_unit_is_optional() {
        let mut json = serde_json::to_value(Stock::default()).unwrap();
        json["unit"] = serde_json::Value::Null;
        let parsed: Stock = serde_json::from_value(json).unwrap();
        assert!(parsed.unit.is_none());
    }
}
