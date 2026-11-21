#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::{error::ValidationError, storage::*, supplier::Supplier};

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
        assert_eq!(storage.storage_comment.as_deref(), Some("line 1\n line 2"));
        assert_eq!(storage.storage_reference.as_deref(), Some("A 12"));
        assert_eq!(storage.storage_batch_number, None);
    }

    #[test]
    fn test_sanitize_and_validate_numbers() {
        let mut storage = Storage {
            storage_quantity: Some(0.0),
            storage_concentration: Some(1.5),
            storage_number_of_bag: Some(1),
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
}
