#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::unittype::*;

    #[test]
    fn test_serde_matches_string_representation() {
        for (unit_type, canonical, legacy) in [
            (UnitType::Quantity, "quantity", "Quantity"),
            (UnitType::Concentration, "concentration", "Concentration"),
            (UnitType::Temperature, "temperature", "Temperature"),
            (
                UnitType::MolecularWeight,
                "molecular_weight",
                "MolecularWeight",
            ),
        ] {
            assert_eq!(unit_type.to_string(), canonical);
            assert_eq!(UnitType::from_str(canonical).unwrap(), unit_type);
            assert_eq!(
                serde_json::to_value(unit_type).unwrap(),
                serde_json::json!(canonical)
            );

            for input in [canonical, legacy] {
                let parsed: UnitType = serde_json::from_value(serde_json::json!(input)).unwrap();
                assert_eq!(parsed, unit_type);
                assert_eq!(
                    serde_json::to_value(parsed).unwrap(),
                    serde_json::json!(canonical)
                );
            }
        }
    }

    #[test]
    fn test_serde_rejects_unknown_unit_type() {
        assert!(serde_json::from_str::<UnitType>("\"invalid\"").is_err());
    }

    #[test]
    fn test_display_quantity() {
        let unit_type = UnitType::Quantity;
        assert_eq!(unit_type.to_string(), "quantity");
    }

    #[test]
    fn test_display_concentration() {
        let unit_type = UnitType::Concentration;
        assert_eq!(unit_type.to_string(), "concentration");
    }

    #[test]
    fn test_display_temperature() {
        let unit_type = UnitType::Temperature;
        assert_eq!(unit_type.to_string(), "temperature");
    }

    #[test]
    fn test_display_molecular_weight() {
        let unit_type = UnitType::MolecularWeight;
        assert_eq!(unit_type.to_string(), "molecular_weight");
    }

    #[test]
    fn test_from_str_quantity() {
        let unit_type = UnitType::from_str("quantity").unwrap();
        assert_eq!(unit_type, UnitType::Quantity);
    }

    #[test]
    fn test_from_str_concentration() {
        let unit_type = UnitType::from_str("concentration").unwrap();
        assert_eq!(unit_type, UnitType::Concentration);
    }

    #[test]
    fn test_from_str_temperature() {
        let unit_type = UnitType::from_str("temperature").unwrap();
        assert_eq!(unit_type, UnitType::Temperature);
    }

    #[test]
    fn test_from_str_molecular_weight() {
        let unit_type = UnitType::from_str("molecular_weight").unwrap();
        assert_eq!(unit_type, UnitType::MolecularWeight);
    }

    #[test]
    fn test_from_str_invalid() {
        let unit_type = UnitType::from_str("invalid");
        assert!(unit_type.is_err());
    }

    #[test]
    fn test_check_unit() {
        let unit_type_of = |unit_id: u64| match unit_id {
            1 => Some(UnitType::Temperature),
            2 => Some(UnitType::Quantity),
            _ => None,
        };
        let unit_with_id = |unit_id: Option<u64>| crate::unit::Unit {
            unit_id,
            ..Default::default()
        };

        // No unit: nothing to check.
        assert!(
            UnitType::Temperature
                .check_unit(None, "unit_temperature", &unit_type_of)
                .is_ok()
        );
        assert!(
            UnitType::Temperature
                .check_unit(
                    Some(&unit_with_id(Some(1))),
                    "unit_temperature",
                    &unit_type_of
                )
                .is_ok()
        );

        // Wrong type, unknown id and no id are rejected.
        for unit_id in [Some(2), Some(9), None] {
            let error = UnitType::Temperature
                .check_unit(
                    Some(&unit_with_id(unit_id)),
                    "unit_temperature",
                    &unit_type_of,
                )
                .unwrap_err();
            assert!(
                matches!(
                    error,
                    crate::error::ValidationError::InvalidUnit {
                        field: "unit_temperature",
                        expected: UnitType::Temperature,
                    }
                ),
                "{unit_id:?}"
            );
        }
    }
}
