#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::classofcompound::*;

    #[test]
    fn test_class_of_compound_searchable() {
        let class_of_compound = ClassOfCompound {
            match_exact_search: true,
            class_of_compound_id: Some(1),
            class_of_compound_label: "test".to_string(),
        };

        let mut created_class_of_compound = class_of_compound.create();
        assert!(!created_class_of_compound.match_exact_search);
        assert_eq!(created_class_of_compound.class_of_compound_id, None);
        assert_eq!(
            created_class_of_compound.class_of_compound_label,
            String::new()
        );

        created_class_of_compound.set_exact_search(true);
        assert!(created_class_of_compound.get_exact_search());

        assert_eq!(
            created_class_of_compound.get_table_name(),
            "class_of_compound"
        );
        assert_eq!(
            created_class_of_compound.get_id_field_name(),
            "class_of_compound_id"
        );
        assert_eq!(
            created_class_of_compound.get_text_field_name(),
            "class_of_compound_label"
        );

        created_class_of_compound.set_id_field(2);
        assert_eq!(created_class_of_compound.get_id(), Some(2));

        created_class_of_compound.set_text_field("test2");
        assert_eq!(created_class_of_compound.get_text(), "test2");
    }

    #[test]
    fn test_create_defaults() {
        let class_of_compound = ClassOfCompound::default();
        let created = class_of_compound.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.class_of_compound_id, None);
        assert_eq!(created.class_of_compound_label, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut class_of_compound = ClassOfCompound::default();
        class_of_compound.set_exact_search(true);
        assert!(class_of_compound.get_exact_search());
        class_of_compound.set_exact_search(false);
        assert!(!class_of_compound.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let class_of_compound = ClassOfCompound::default();
        assert_eq!(class_of_compound.get_table_name(), "class_of_compound");
    }

    #[test]
    fn test_get_id_field_name() {
        let class_of_compound = ClassOfCompound::default();
        assert_eq!(class_of_compound.get_id_field_name(), "class_of_compound_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut class_of_compound = ClassOfCompound::default();
        class_of_compound.set_id_field(42);
        assert_eq!(class_of_compound.get_id(), Some(42));
        class_of_compound.set_id_field(0);
        assert_eq!(class_of_compound.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut class_of_compound = ClassOfCompound::default();
        class_of_compound.set_text_field("Organic Compounds");
        assert_eq!(class_of_compound.get_text(), "Organic Compounds");
        class_of_compound.set_text_field("");
        assert_eq!(class_of_compound.get_text(), "");
    }

    #[test]
    fn test_sanitize_and_validate_class_of_compound() {
        let mut class_of_compound = ClassOfCompound {
            match_exact_search: false,
            class_of_compound_id: Some(1),
            class_of_compound_label: "  Organic Compounds  ".to_string(),
        };
        assert!(class_of_compound.sanitize_and_validate().is_ok());
        assert_eq!(
            class_of_compound.class_of_compound_label,
            "Organic Compounds"
        );

        let mut class_of_compound = ClassOfCompound {
            match_exact_search: false,
            class_of_compound_id: Some(2),
            class_of_compound_label: "  Inorganic Compounds  ".to_string(),
        };
        assert!(class_of_compound.sanitize_and_validate().is_ok());
        assert_eq!(
            class_of_compound.class_of_compound_label,
            "Inorganic Compounds"
        );

        let mut class_of_compound = ClassOfCompound {
            match_exact_search: false,
            class_of_compound_id: Some(3),
            class_of_compound_label: "  Organic   Compounds  ".to_string(),
        };
        assert!(class_of_compound.sanitize_and_validate().is_ok());
        assert_eq!(
            class_of_compound.class_of_compound_label,
            "Organic Compounds"
        );
    }

    #[test]
    fn test_sanitize_and_validate_empty_class_of_compound() {
        let mut class_of_compound = ClassOfCompound {
            match_exact_search: false,
            class_of_compound_id: Some(6),
            class_of_compound_label: String::default(),
        };
        assert!(class_of_compound.sanitize_and_validate().is_err());
    }
}
