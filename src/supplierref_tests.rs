#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ValidationError;
    use crate::supplier::Supplier;
    use crate::supplierref::*;
    use crate::test_helpers::{
        assert_serde_rejects_missing, assert_serde_round_trip, assert_validation_error,
    };
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "match_exact_search": true,
            "supplier_ref_id": 1,
            "supplier_ref_label": "REF-1",
            "supplier": {"match_exact_search": false, "supplier_id": 2, "supplier_label": "Acme"}
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<SupplierRef>(&sample());
        assert_eq!(value.supplier.supplier_id, Some(2));
    }

    #[test]
    fn test_serde_defaults() {
        let value: SupplierRef = serde_json::from_value(json!({
            "supplier_ref_label": "REF-1",
            "supplier": {"supplier_label": "Acme"}
        }))
        .unwrap();
        assert!(!value.match_exact_search);
        assert_eq!(value.supplier_ref_id, None);
        assert!(!value.supplier.match_exact_search);
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<SupplierRef>(&sample(), &["supplier_ref_label", "supplier"]);
    }

    // sanitize_and_validate.

    #[test]
    fn test_sanitize_and_validate() {
        // The label and the label of a new supplier (without id) are cleaned.
        let mut value = SupplierRef {
            supplier_ref_label: "  REF   1 ".to_string(),
            supplier: Supplier {
                supplier_label: "  Acme   Corp ".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.supplier_ref_label, "REF 1");
        assert_eq!(value.supplier.supplier_label, "Acme Corp");

        // An existing supplier is not validated.
        let mut value = SupplierRef {
            supplier_ref_label: "REF".to_string(),
            supplier: Supplier {
                supplier_id: Some(3),
                ..Default::default()
            },
            ..Default::default()
        };
        value.sanitize_and_validate().unwrap();
        assert_eq!(value.supplier.supplier_label, "");
    }

    #[test]
    fn test_sanitize_and_validate_errors() {
        let mut value = SupplierRef {
            supplier_ref_label: "   ".to_string(),
            supplier: Supplier {
                supplier_label: "Acme".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        let error = assert_validation_error(value.sanitize_and_validate(), "supplier_ref_label");
        assert!(matches!(error, ValidationError::Empty { .. }));

        // A new supplier with an empty label is rejected.
        let mut value = SupplierRef {
            supplier_ref_label: "REF".to_string(),
            ..Default::default()
        };
        let error = assert_validation_error(value.sanitize_and_validate(), "supplier_label");
        assert!(matches!(error, ValidationError::Empty { .. }));
    }
}
