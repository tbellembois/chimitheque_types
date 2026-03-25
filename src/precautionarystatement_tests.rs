#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::precautionarystatement::*;

    #[test]
    fn test_precautionary_statement_searchable() {
        let precautionary_statement = PrecautionaryStatement {
            match_exact_search: true,
            precautionary_statement_id: Some(1),
            precautionary_statement_label: "test".to_string(),
            ..Default::default()
        };

        let mut created_precautionary_statement = precautionary_statement.create();
        assert!(!created_precautionary_statement.match_exact_search);
        assert_eq!(
            created_precautionary_statement.precautionary_statement_id,
            None
        );
        assert_eq!(
            created_precautionary_statement.precautionary_statement_label,
            String::new()
        );

        created_precautionary_statement.set_exact_search(true);
        assert!(created_precautionary_statement.get_exact_search());

        assert_eq!(
            created_precautionary_statement.get_table_name(),
            "precautionary_statement"
        );
        assert_eq!(
            created_precautionary_statement.get_id_field_name(),
            "precautionary_statement_id"
        );
        assert_eq!(
            created_precautionary_statement.get_text_field_name(),
            "precautionary_statement_reference"
        );

        created_precautionary_statement.set_id_field(2);
        assert_eq!(created_precautionary_statement.get_id(), Some(2));

        created_precautionary_statement.set_text_field("test2");
        assert_eq!(created_precautionary_statement.get_text(), "test2");
    }
}
