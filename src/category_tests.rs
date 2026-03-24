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
}
