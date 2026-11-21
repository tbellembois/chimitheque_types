#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::{error::ValidationError, storage::*, supplier::Supplier};

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
}
