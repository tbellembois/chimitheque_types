#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use std::str::FromStr;

    use crate::unittype::*;
    use serde_json::json;

    /// Each unit type with its canonical and legacy names.
    const TYPES: [(UnitType, &str, &str); 4] = [
        (UnitType::Quantity, "quantity", "Quantity"),
        (UnitType::Concentration, "concentration", "Concentration"),
        (UnitType::Temperature, "temperature", "Temperature"),
        (
            UnitType::MolecularWeight,
            "molecular_weight",
            "MolecularWeight",
        ),
    ];

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        for (unit_type, canonical, legacy) in TYPES {
            assert_eq!(serde_json::to_value(unit_type).unwrap(), json!(canonical));
            // The legacy PascalCase names are accepted too.
            for input in [canonical, legacy] {
                assert_eq!(
                    serde_json::from_value::<UnitType>(json!(input)).unwrap(),
                    unit_type
                );
            }
        }
    }

    #[test]
    fn test_serde_rejects_invalid() {
        for input in ["", "invalid", "QUANTITY", "molecularWeight"] {
            assert!(
                serde_json::from_value::<UnitType>(json!(input)).is_err(),
                "{input}"
            );
        }
    }

    // Other functions.

    #[test]
    fn test_default() {
        assert_eq!(UnitType::default(), UnitType::Quantity);
    }

    #[test]
    fn test_display_and_from_str() {
        for (unit_type, canonical, _) in TYPES {
            assert_eq!(unit_type.to_string(), canonical);
            assert_eq!(UnitType::from_str(canonical).unwrap(), unit_type);
        }
    }

    #[test]
    fn test_from_str_rejects_invalid() {
        // Unlike deserialization, the legacy names are rejected.
        for input in ["", "invalid", "Quantity", "MolecularWeight"] {
            assert!(UnitType::from_str(input).is_err(), "{input}");
        }
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
