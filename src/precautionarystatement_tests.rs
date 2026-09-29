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

    #[test]
    fn test_create_defaults() {
        let precautionary_statement = PrecautionaryStatement::default();
        let created = precautionary_statement.create();
        assert!(!created.match_exact_search);
        assert_eq!(created.precautionary_statement_id, None);
        assert_eq!(created.precautionary_statement_label, String::new());
        assert_eq!(created.precautionary_statement_reference, String::new());
    }

    #[test]
    fn test_set_and_get_exact_search() {
        let mut precautionary_statement = PrecautionaryStatement::default();
        precautionary_statement.set_exact_search(true);
        assert!(precautionary_statement.get_exact_search());
        precautionary_statement.set_exact_search(false);
        assert!(!precautionary_statement.get_exact_search());
    }

    #[test]
    fn test_get_table_name() {
        let precautionary_statement = PrecautionaryStatement::default();
        assert_eq!(precautionary_statement.get_table_name(), "precautionary_statement");
    }

    #[test]
    fn test_get_id_field_name() {
        let precautionary_statement = PrecautionaryStatement::default();
        assert_eq!(precautionary_statement.get_id_field_name(), "precautionary_statement_id");
    }

    #[test]
    fn test_set_and_get_id() {
        let mut precautionary_statement = PrecautionaryStatement::default();
        precautionary_statement.set_id_field(42);
        assert_eq!(precautionary_statement.get_id(), Some(42));
        precautionary_statement.set_id_field(0);
        assert_eq!(precautionary_statement.get_id(), Some(0));
    }

    #[test]
    fn test_set_and_get_text() {
        let mut precautionary_statement = PrecautionaryStatement::default();
        precautionary_statement.set_text_field("P210");
        assert_eq!(precautionary_statement.get_text(), "P210");
        precautionary_statement.set_text_field("");
        assert_eq!(precautionary_statement.get_text(), "");
    }
}
