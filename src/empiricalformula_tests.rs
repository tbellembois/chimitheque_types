#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::empiricalformula::*;

    #[test]
    fn test_empirical_formula_searchable() {
        let empirical_formula = EmpiricalFormula {
            match_exact_search: true,
            empirical_formula_id: Some(1),
            empirical_formula_label: "test".to_string(),
        };

        let mut created_empirical_formula = empirical_formula.create();
        assert!(!created_empirical_formula.match_exact_search);
        assert_eq!(created_empirical_formula.empirical_formula_id, None);
        assert_eq!(
            created_empirical_formula.empirical_formula_label,
            String::new()
        );

        created_empirical_formula.set_exact_search(true);
        assert!(created_empirical_formula.get_exact_search());

        assert_eq!(
            created_empirical_formula.get_table_name(),
            "empirical_formula"
        );
        assert_eq!(
            created_empirical_formula.get_id_field_name(),
            "empirical_formula_id"
        );
        assert_eq!(
            created_empirical_formula.get_text_field_name(),
            "empirical_formula_label"
        );

        created_empirical_formula.set_id_field(2);
        assert_eq!(created_empirical_formula.get_id(), Some(2));

        created_empirical_formula.set_text_field("test2");
        assert_eq!(created_empirical_formula.get_text(), "test2");
    }

    #[test]
    fn test_sanitize_and_validate_empirical_formula() {
        let mut empirical_formula = EmpiricalFormula {
            match_exact_search: false,
            empirical_formula_id: Some(1),
            empirical_formula_label: "H2O  ".to_string(),
        };
        assert!(empirical_formula.sanitize_and_validate().is_ok());
        assert_eq!(empirical_formula.empirical_formula_label, "H2O");

        let mut empirical_formula = EmpiricalFormula {
            match_exact_search: false,
            empirical_formula_id: Some(2),
            empirical_formula_label: "C6H12O6".to_string(),
        };
        assert!(empirical_formula.sanitize_and_validate().is_ok());
        assert_eq!(empirical_formula.empirical_formula_label, "C6H12O6");
    }

    #[test]
    fn test_sanitize_and_validate_invalid_empirical_formula() {
        let mut empirical_formula = EmpiricalFormula {
            match_exact_search: false,
            empirical_formula_id: Some(4),
            empirical_formula_label: "X123X".to_string(),
        };
        assert!(empirical_formula.sanitize_and_validate().is_err());
    }

    #[test]
    fn test_sanitize_and_validate_empty_empirical_formula() {
        let mut empirical_formula = EmpiricalFormula {
            match_exact_search: false,
            empirical_formula_id: Some(5),
            empirical_formula_label: String::default(),
        };
        assert!(empirical_formula.sanitize_and_validate().is_err());
    }
}
