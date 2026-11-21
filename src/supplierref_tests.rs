#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::supplier::Supplier;
    use crate::supplierref::*;

    #[test]
    fn test_match_exact_search_serde_defaults() -> Result<(), serde_json::Error> {
        let mut payload = serde_json::json!({
            "supplier_ref_label": "S-001",
            "supplier": {"supplier_label": "Supplier"}
        });
        let value: SupplierRef = serde_json::from_value(payload.clone())?;
        assert!(!value.match_exact_search);
        assert_eq!(
            serde_json::to_value(&value)?.get("match_exact_search"),
            Some(&serde_json::json!(false))
        );
        assert!(!value.supplier.match_exact_search);

        for expected in [true, false] {
            payload["match_exact_search"] = serde_json::json!(expected);
            let value: SupplierRef = serde_json::from_value(payload.clone())?;
            assert_eq!(value.match_exact_search, expected);
            assert_eq!(
                serde_json::to_value(&value)?.get("match_exact_search"),
                Some(&serde_json::json!(expected))
            );
        }

        for required in ["supplier_ref_label", "supplier"] {
            let mut missing_required = payload.clone();
            if let Some(object) = missing_required.as_object_mut() {
                object.remove(required);
            }
            assert!(serde_json::from_value::<SupplierRef>(missing_required).is_err());
        }
        Ok(())
    }

    fn assert_empty(result: Result<(), crate::error::ValidationError>, expected_field: &str) {
        let error = result.unwrap_err();
        assert!(
            matches!(error, crate::error::ValidationError::Empty { field } if field == expected_field),
            "{error:?}"
        );
    }

    #[test]
    fn test_sanitize_and_validate() {
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

        let mut value = SupplierRef {
            supplier_ref_label: "   ".to_string(),
            ..Default::default()
        };
        assert_empty(value.sanitize_and_validate(), "supplier_ref_label");

        // A new supplier with an empty label is rejected.
        let mut value = SupplierRef {
            supplier_ref_label: "REF".to_string(),
            ..Default::default()
        };
        assert_empty(value.sanitize_and_validate(), "supplier_label");

        // An existing supplier is left unchanged.
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
}
