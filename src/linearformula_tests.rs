#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::linearformula::*;

    #[test]
    fn test_linear_formula_searchable() {
        let linear_formula = LinearFormula {
            match_exact_search: true,
            linear_formula_id: Some(1),
            linear_formula_label: "test".to_string(),
        };

        let mut created_linear_formula = linear_formula.create();
        assert!(!created_linear_formula.match_exact_search);
        assert_eq!(created_linear_formula.linear_formula_id, None);
        assert_eq!(created_linear_formula.linear_formula_label, String::new());

        created_linear_formula.set_exact_search(true);
        assert!(created_linear_formula.get_exact_search());

        assert_eq!(created_linear_formula.get_table_name(), "linear_formula");
        assert_eq!(
            created_linear_formula.get_id_field_name(),
            "linear_formula_id"
        );
        assert_eq!(
            created_linear_formula.get_text_field_name(),
            "linear_formula_label"
        );

        created_linear_formula.set_id_field(2);
        assert_eq!(created_linear_formula.get_id(), Some(2));

        created_linear_formula.set_text_field("test2");
        assert_eq!(created_linear_formula.get_text(), "test2");
    }

    #[test]
    fn test_sanitize_and_validate_linear_formula() {
        let mut linear_formula = LinearFormula {
            match_exact_search: false,
            linear_formula_id: Some(1),
            linear_formula_label: "H2O  ".to_string(),
        };
        assert!(linear_formula.sanitize_and_validate().is_ok());
        assert_eq!(linear_formula.linear_formula_label, "H2O");

        let mut linear_formula = LinearFormula {
            match_exact_search: false,
            linear_formula_id: Some(2),
            linear_formula_label: "C6H12O6".to_string(),
        };
        assert!(linear_formula.sanitize_and_validate().is_ok());
        assert_eq!(linear_formula.linear_formula_label, "C6H12O6");
    }

    #[test]
    fn test_sanitize_and_validate_invalid_linear_formula() {
        let mut linear_formula = LinearFormula {
            match_exact_search: false,
            linear_formula_id: Some(4),
            linear_formula_label: "X123X".to_string(),
        };
        assert!(linear_formula.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_empty_linear_formula() {
        let mut linear_formula = LinearFormula {
            match_exact_search: false,
            linear_formula_id: Some(5),
            linear_formula_label: String::default(),
        };
        assert!(linear_formula.sanitize_and_validate().is_err());
    }
}
