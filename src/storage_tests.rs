#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::test_helpers::{assert_serde_rejects_missing, assert_value_round_trip};
    use crate::{error::ValidationError, storage::*, supplier::Supplier, unit::Unit};

    /// Makes a storage invalid, and the field expected in the error.
    type InvalidCase = (fn(&mut Storage), &'static str);

    fn storage_with_supplier(supplier_id: Option<u64>, supplier_label: &str) -> Storage {
        Storage {
            supplier: Some(Supplier {
                supplier_id,
                supplier_label: supplier_label.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    // Serialization and deserialization.

    fn date(text: &str) -> chrono::DateTime<chrono::Utc> {
        text.parse().unwrap()
    }

    fn serde_sample() -> Storage {
        Storage {
            storage_id: Some(1),
            storage_creation_date: date("2026-01-02T03:04:05Z"),
            storage_modification_date: date("2026-01-03T03:04:05Z"),
            storage_entry_date: Some(date("2026-01-02T00:00:00Z")),
            storage_comment: Some("line 1\nline 2".to_string()),
            storage_quantity: Some(2.5),
            unit_quantity: Some(Unit {
                unit_id: Some(2),
                unit_label: "L".to_string(),
                unit_multiplier: 1.0,
                ..Default::default()
            }),
            storage_to_destroy: true,
            storage_number_of_bag: Some(3),
            supplier: Some(Supplier {
                supplier_id: Some(4),
                supplier_label: "Acme".to_string(),
                ..Default::default()
            }),
            storage_hc: 5,
            ..Default::default()
        }
    }

    #[test]
    fn test_serde() {
        let json = assert_value_round_trip(&serde_sample());
        assert_eq!(json["storage_creation_date"], "2026-01-02T03:04:05Z");
        assert_eq!(json["storage_entry_date"], "2026-01-02T00:00:00Z");
        assert_eq!(json["storage_comment"], "line 1\nline 2");
        assert_eq!(json["storage_quantity"], 2.5);
        assert_eq!(json["unit_quantity"]["unit_label"], "L");
        assert_eq!(json["storage_to_destroy"], true);
        assert_eq!(json["supplier"]["supplier_label"], "Acme");
        assert_eq!(json["storage_hc"], 5);
    }

    #[test]
    fn test_serde_defaults() {
        let value: Storage = serde_json::from_value(serde_json::json!({
            "storage_creation_date": "2026-01-02T03:04:05Z",
            "storage_modification_date": "2026-01-02T03:04:05Z",
            "product": {"product_type": "chem", "name": {"name_label": "ETHANOL"}},
            "store_location": {"store_location_name": "shelf", "store_location_can_store": true}
        }))
        .unwrap();
        assert_eq!(value.storage_id, None);
        assert_eq!(value.storage_entry_date, None);
        assert!(!value.storage_to_destroy);
        assert!(!value.storage_archive);
        assert_eq!(value.person.person_id, None);
        assert_eq!(value.storage_hc, 0);
        assert!(value.supplier.is_none());
        assert!(value.borrowing.is_none());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        let json = assert_value_round_trip(&serde_sample());
        assert_serde_rejects_missing::<Storage>(
            &json,
            &[
                "storage_creation_date",
                "storage_modification_date",
                "product",
                "store_location",
            ],
        );
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate_without_supplier() {
        let mut storage = Storage::default();
        assert!(storage.sanitize_and_validate().is_ok());
    }

    #[test]
    fn test_sanitize_and_validate_cleans_new_supplier() {
        let mut storage = storage_with_supplier(None, "  Acme   Chemicals ");
        storage.sanitize_and_validate().unwrap();
        assert_eq!(storage.supplier.unwrap().supplier_label, "Acme Chemicals");
    }

    #[test]
    fn test_sanitize_and_validate_rejects_empty_new_supplier() {
        let mut storage = storage_with_supplier(None, "   ");
        let error = storage.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::Empty {
                field: "supplier_label"
            }
        ));
    }

    #[test]
    fn test_sanitize_and_validate_keeps_existing_supplier() {
        let mut storage = storage_with_supplier(Some(4), "");
        storage.sanitize_and_validate().unwrap();
        assert_eq!(storage.supplier.unwrap().supplier_label, "");
    }

    #[test]
    fn test_sanitize_and_validate_optional_text() {
        let mut storage = Storage {
            storage_comment: Some("  line 1\n line 2 ".to_string()),
            storage_reference: Some(" A   12 ".to_string()),
            storage_batch_number: Some("   ".to_string()),
            ..Default::default()
        };
        storage.sanitize_and_validate().unwrap();
        assert_eq!(storage.storage_comment.as_deref(), Some("line 1\nline 2"));
        assert_eq!(storage.storage_reference.as_deref(), Some("A 12"));
        assert_eq!(storage.storage_batch_number, None);
    }

    #[test]
    fn test_sanitize_and_validate_numbers() {
        let mut storage = Storage {
            storage_quantity: Some(0.0),
            storage_concentration: Some(1.5),
            storage_number_of_bag: Some(1),
            unit_quantity: Some(Unit::default()),
            unit_concentration: Some(Unit::default()),
            ..Default::default()
        };
        storage.sanitize_and_validate().unwrap();

        let cases: [InvalidCase; 4] = [
            (|s| s.storage_quantity = Some(-1.0), "storage_quantity"),
            (
                |s| s.storage_concentration = Some(f64::NAN),
                "storage_concentration",
            ),
            (
                |s| s.storage_number_of_bag = Some(0),
                "storage_number_of_bag",
            ),
            (
                |s| s.storage_number_of_carton = Some(0),
                "storage_number_of_carton",
            ),
        ];
        for (set_invalid, expected_field) in cases {
            let mut storage = Storage::default();
            set_invalid(&mut storage);
            let error = storage.sanitize_and_validate().unwrap_err();
            assert!(
                matches!(error, ValidationError::OutOfRange { field, .. } if field == expected_field),
                "{error:?}"
            );
        }
    }

    #[test]
    fn test_sanitize_and_validate_dates() {
        let entry = chrono::DateTime::from_timestamp(1_700_000_000, 0);
        let later = chrono::DateTime::from_timestamp(1_700_000_100, 0);

        for (entry_date, exit_date) in
            [(entry, later), (entry, entry), (None, later), (entry, None)]
        {
            let mut storage = Storage {
                storage_entry_date: entry_date,
                storage_exit_date: exit_date,
                ..Default::default()
            };
            storage.sanitize_and_validate().unwrap();
        }

        let mut storage = Storage {
            storage_entry_date: later,
            storage_exit_date: entry,
            ..Default::default()
        };
        let error = storage.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::OutOfRange {
                field: "storage_exit_date",
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "storage_exit_date: must be on or after storage_entry_date"
        );
    }

    #[test]
    fn test_sanitize_and_validate_borrowing() {
        let mut storage = Storage {
            borrowing: Some(crate::borrowing::Borrowing {
                borrowing_comment: Some("  ".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        storage.sanitize_and_validate().unwrap();
        assert_eq!(storage.borrowing.unwrap().borrowing_comment, None);
    }

    #[test]
    fn test_sanitize_and_validate_units_required() {
        for (storage, expected_field) in [
            (
                Storage {
                    storage_quantity: Some(1.0),
                    ..Default::default()
                },
                "unit_quantity",
            ),
            (
                Storage {
                    storage_concentration: Some(1.0),
                    ..Default::default()
                },
                "unit_concentration",
            ),
        ] {
            let mut storage = storage;
            let error = storage.sanitize_and_validate().unwrap_err();
            assert!(
                matches!(error, ValidationError::Required { field, .. } if field == expected_field),
                "{error:?}"
            );
        }
    }

    #[test]
    fn test_sanitize_and_validate_opening_date() {
        let entry = chrono::DateTime::from_timestamp(1_700_000_000, 0);
        let later = chrono::DateTime::from_timestamp(1_700_000_100, 0);

        let mut storage = Storage {
            storage_entry_date: entry,
            storage_opening_date: later,
            ..Default::default()
        };
        storage.sanitize_and_validate().unwrap();

        let mut storage = Storage {
            storage_entry_date: later,
            storage_opening_date: entry,
            ..Default::default()
        };
        let error = storage.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::OutOfRange {
                field: "storage_opening_date",
                ..
            }
        ));
    }

    /// Unit types as in the base data: °C, g/mol, mL and mg/L.
    fn base_unit_type(unit_id: u64) -> Option<crate::unittype::UnitType> {
        use crate::unittype::UnitType;
        match unit_id {
            14 => Some(UnitType::Temperature),
            24 => Some(UnitType::MolecularWeight),
            2 => Some(UnitType::Quantity),
            21 => Some(UnitType::Concentration),
            _ => None,
        }
    }

    fn unit_with_id(unit_id: Option<u64>) -> Unit {
        // The request may carry only the id: unit_type keeps its default value.
        Unit {
            unit_id,
            ..Default::default()
        }
    }

    // Other functions.

    #[test]
    fn test_validate_unit_types() {
        let mut storage = Storage::default();
        storage.validate_unit_types(base_unit_type).unwrap();

        storage.unit_quantity = Some(unit_with_id(Some(2)));
        storage.unit_concentration = Some(unit_with_id(Some(21)));
        storage.validate_unit_types(base_unit_type).unwrap();

        // mg/L (a concentration unit) as quantity unit.
        storage.unit_quantity = Some(unit_with_id(Some(21)));
        let error = storage.validate_unit_types(base_unit_type).unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidUnit {
                field: "unit_quantity",
                ..
            }
        ));
        assert_eq!(error.to_string(), "unit_quantity: must be a quantity unit");

        // mL as concentration unit.
        let mut storage = Storage {
            unit_concentration: Some(unit_with_id(Some(2))),
            ..Default::default()
        };
        let error = storage.validate_unit_types(base_unit_type).unwrap_err();
        assert_eq!(
            error.to_string(),
            "unit_concentration: must be a concentration unit"
        );
        storage.unit_concentration = None;
        storage.validate_unit_types(base_unit_type).unwrap();
    }

    #[test]
    fn test_sanitize_and_validate_expiration_date() {
        let entry = chrono::DateTime::from_timestamp(1_700_000_000, 0);
        let later = chrono::DateTime::from_timestamp(1_700_000_100, 0);

        for (entry_date, expiration_date) in [(entry, later), (entry, entry), (None, entry)] {
            let mut storage = Storage {
                storage_entry_date: entry_date,
                storage_expiration_date: expiration_date,
                ..Default::default()
            };
            storage.sanitize_and_validate().unwrap();
        }

        // Stock already expired when it enters is refused.
        let mut storage = Storage {
            storage_entry_date: later,
            storage_expiration_date: entry,
            ..Default::default()
        };
        let error = storage.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::OutOfRange {
                field: "storage_expiration_date",
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "storage_expiration_date: must be on or after storage_entry_date"
        );
    }
}
