#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::category::*;

    #[test]
    fn test_category_searchable() {
        let category = Category {
            match_exact_search: true,
            category_id: Some(1),
            category_label: "test".to_string(),
        };

        let mut created_category = category.create();
        assert!(!created_category.match_exact_search);
        assert_eq!(created_category.category_id, None);
        assert_eq!(created_category.category_label, String::new());

        created_category.set_exact_search(true);
        assert!(created_category.get_exact_search());

        assert_eq!(created_category.get_table_name(), "category");
        assert_eq!(created_category.get_id_field_name(), "category_id");
        assert_eq!(created_category.get_text_field_name(), "category_label");

        created_category.set_id_field(2);
        assert_eq!(created_category.get_id(), Some(2));

        created_category.set_text_field("test2");
        assert_eq!(created_category.get_text(), "test2");
    }

    #[test]
    fn test_create() {
        let category = Category {
            match_exact_search: true,
            category_id: Some(1),
            category_label: "test".to_string(),
        };
        let created = category.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.category_id, None);
        assert_eq!(created.category_label, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut category = Category::default();
        category.set_exact_search(true);
        assert!(category.get_exact_search());
        category.set_exact_search(false);
        assert!(!category.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let category = Category::default();
        assert_eq!(category.get_table_name(), "category");
    }

    #[test]
    fn test_get_id_field_name() {
        let category = Category::default();
        assert_eq!(category.get_id_field_name(), "category_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut category = Category::default();
        category.set_id_field(42);
        assert_eq!(category.get_id(), Some(42));
        category.set_id_field(0);
        assert_eq!(category.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut category = Category::default();
        category.set_text_field("Development");
        assert_eq!(category.get_text(), "Development");
        category.set_text_field("");
        assert_eq!(category.get_text(), "");
    }

    #[test]
    fn test_sanitize_and_validate_category() {
        let mut category = Category {
            match_exact_search: false,
            category_id: Some(1),
            category_label: "  Development  ".to_string(),
        };
        assert!(category.sanitize_and_validate().is_ok());
        assert_eq!(category.category_label, "Development");

        let mut category = Category {
            match_exact_search: false,
            category_id: Some(2),
            category_label: "  Chemistry  ".to_string(),
        };
        assert!(category.sanitize_and_validate().is_ok());
        assert_eq!(category.category_label, "Chemistry");

        let mut category = Category {
            match_exact_search: false,
            category_id: Some(3),
            category_label: "   A  B   ".to_string(),
        };
        assert!(category.sanitize_and_validate().is_ok());
        assert_eq!(category.category_label, "A B");
    }

    #[test]
    fn test_sanitize_and_validate_empty_category() {
        let mut category = Category {
            match_exact_search: false,
            category_id: Some(6),
            category_label: String::default(),
        };
        assert!(category.sanitize_and_validate().is_err());
    }
}
