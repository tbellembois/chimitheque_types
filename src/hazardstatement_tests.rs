#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::hazardstatement::*;

    #[test]
    fn test_hazard_statement_searchable() {
        let hazard_statement = HazardStatement {
            match_exact_search: true,
            hazard_statement_id: Some(1),
            hazard_statement_label: "test".to_string(),
            ..Default::default()
        };

        let mut created_hazard_statement = hazard_statement.create();
        assert!(!created_hazard_statement.match_exact_search);
        assert_eq!(created_hazard_statement.hazard_statement_id, None);
        assert_eq!(
            created_hazard_statement.hazard_statement_label,
            String::new()
        );

        created_hazard_statement.set_exact_search(true);
        assert!(created_hazard_statement.get_exact_search());

        assert_eq!(
            created_hazard_statement.get_table_name(),
            "hazard_statement"
        );
        assert_eq!(
            created_hazard_statement.get_id_field_name(),
            "hazard_statement_id"
        );
        assert_eq!(
            created_hazard_statement.get_text_field_name(),
            "hazard_statement_reference"
        );

        created_hazard_statement.set_id_field(2);
        assert_eq!(created_hazard_statement.get_id(), Some(2));

        created_hazard_statement.set_text_field("test2");
        assert_eq!(created_hazard_statement.get_text(), "test2");
    }
}
