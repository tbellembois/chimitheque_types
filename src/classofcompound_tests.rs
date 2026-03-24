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
}
