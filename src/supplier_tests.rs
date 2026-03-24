#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::supplier::*;

    #[test]
    fn test_supplier_searchable() {
        let supplier = Supplier {
            match_exact_search: true,
            supplier_id: Some(1),
            supplier_label: "test".to_string(),
        };

        let mut created_supplier = supplier.create();
        assert!(!created_supplier.match_exact_search);
        assert_eq!(created_supplier.supplier_id, None);
        assert_eq!(created_supplier.supplier_label, String::new());

        created_supplier.set_exact_search(true);
        assert!(created_supplier.get_exact_search());

        assert_eq!(created_supplier.get_table_name(), "supplier");
        assert_eq!(created_supplier.get_id_field_name(), "supplier_id");
        assert_eq!(created_supplier.get_text_field_name(), "supplier_label");

        created_supplier.set_id_field(2);
        assert_eq!(created_supplier.get_id(), Some(2));

        created_supplier.set_text_field("test2");
        assert_eq!(created_supplier.get_text(), "test2");
    }

    #[test]
    fn test_sanitize_and_validate_leading_trailing_spaces() {
        let mut supplier = Supplier {
            match_exact_search: false,
            supplier_id: Some(1),
            supplier_label: "  hello world  ".to_string(),
        };
        assert!(supplier.sanitize_and_validate().is_ok());
        assert_eq!(supplier.supplier_label, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_multiple_spaces() {
        let mut supplier = Supplier {
            match_exact_search: false,
            supplier_id: Some(1),
            supplier_label: "  hello    world  ".to_string(),
        };
        assert!(supplier.sanitize_and_validate().is_ok());
        assert_eq!(supplier.supplier_label, "hello world");
    }

    #[test]
    fn test_sanitize_and_validate_empty_label() {
        let mut supplier = Supplier {
            match_exact_search: false,
            supplier_id: Some(1),
            supplier_label: String::default(),
        };
        assert!(supplier.sanitize_and_validate().is_ok());
        assert_eq!(supplier.supplier_label, "");
    }

    #[test]
    fn test_sanitize_and_validate_no_spaces() {
        let mut supplier = Supplier {
            match_exact_search: false,
            supplier_id: Some(1),
            supplier_label: "hello world".to_string(),
        };
        assert!(supplier.sanitize_and_validate().is_ok());
        assert_eq!(supplier.supplier_label, "hello world");
    }
}
