#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use std::error::Error as _;

    use crate::{
        casnumber::CasNumber, category::Category, cenumber::CeNumber,
        classofcompound::ClassOfCompound, empiricalformula::EmpiricalFormula, entity::Entity,
        error::ValidationError, linearformula::LinearFormula, name::Name, person::Person,
        producer::Producer, product::Product, storelocation::StoreLocation, supplier::Supplier,
        tag::Tag,
    };
    use chimitheque_utils::{
        casnumber::CasNumberError, cenumber::CeNumberError, formula::ToEmpiricalFormulaError,
    };

    fn assert_empty(result: Result<(), ValidationError>, expected_field: &str) {
        let error = result.unwrap_err();
        assert!(
            matches!(error, ValidationError::Empty { field } if field == expected_field),
            "{error:?}"
        );
        assert_eq!(error.field(), expected_field);
        assert_eq!(error.to_string(), format!("{expected_field}: empty input"));
        assert!(error.source().is_none());
    }

    #[test]
    fn test_empty_fields_are_named() {
        assert_empty(
            Category::default().sanitize_and_validate(),
            "category_label",
        );
        assert_empty(
            ClassOfCompound::default().sanitize_and_validate(),
            "class_of_compound_label",
        );
        assert_empty(Name::default().sanitize_and_validate(), "name_label");
        assert_empty(Tag::default().sanitize_and_validate(), "tag_label");
        assert_empty(
            Producer::default().sanitize_and_validate(),
            "producer_label",
        );
        assert_empty(
            Supplier::default().sanitize_and_validate(),
            "supplier_label",
        );
        assert_empty(
            CasNumber::default().sanitize_and_validate(),
            "cas_number_label",
        );
        assert_empty(
            CeNumber::default().sanitize_and_validate(),
            "ce_number_label",
        );
        assert_empty(
            LinearFormula::default().sanitize_and_validate(),
            "linear_formula_label",
        );
        assert_empty(Entity::default().sanitize_and_validate(), "entity_name");
        assert_empty(
            StoreLocation::default().sanitize_and_validate(),
            "store_location_name",
        );
    }

    #[test]
    fn test_invalid_cas_number() {
        let mut cas_number = CasNumber {
            cas_number_label: "7732-18-4".to_string(),
            ..Default::default()
        };
        let error = cas_number.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidCasNumber(CasNumberError::CheckDigitDoesNotMatch)
        ));
        assert_eq!(error.field(), "cas_number_label");
        assert_eq!(
            error.to_string(),
            "cas_number_label: check digit does not match"
        );
        assert_eq!(
            error.source().unwrap().to_string(),
            "check digit does not match"
        );
    }

    #[test]
    fn test_invalid_ce_number() {
        let mut ce_number = CeNumber {
            ce_number_label: "231-791-3".to_string(),
            ..Default::default()
        };
        let error = ce_number.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidCeNumber(CeNumberError::CheckDigitDoesNotMatch)
        ));
        assert_eq!(error.field(), "ce_number_label");
        assert!(error.to_string().starts_with("ce_number_label: "));
        assert!(error.source().is_some());
    }

    #[test]
    fn test_invalid_formulas() {
        let mut empirical_formula = EmpiricalFormula {
            empirical_formula_label: "Xx2".to_string(),
            ..Default::default()
        };
        let error = empirical_formula.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            &error,
            ValidationError::InvalidFormula {
                field: "empirical_formula_label",
                source: ToEmpiricalFormulaError::UnknowAtom(atom),
            } if atom == "Xx"
        ));
        assert_eq!(
            error.to_string(),
            "empirical_formula_label: unknown atom Xx"
        );
        assert!(error.source().is_some());

        // An empty empirical formula is reported by the formula parser.
        let error = EmpiricalFormula::default()
            .sanitize_and_validate()
            .unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidFormula {
                field: "empirical_formula_label",
                source: ToEmpiricalFormulaError::EmptyFormula,
            }
        ));

        let mut linear_formula = LinearFormula {
            linear_formula_label: "CH3Qq".to_string(),
            ..Default::default()
        };
        let error = linear_formula.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidFormula {
                field: "linear_formula_label",
                ..
            }
        ));
        assert!(error.to_string().starts_with("linear_formula_label: "));
    }

    #[test]
    fn test_invalid_email() {
        let mut person = Person {
            person_email: "not an email".to_string(),
            ..Default::default()
        };
        let error = person.sanitize_and_validate().unwrap_err();
        assert!(matches!(error, ValidationError::InvalidEmail(_)));
        assert_eq!(error.field(), "person_email");
        assert!(error.to_string().starts_with("person_email: "));
        assert!(error.source().is_some());
    }

    #[test]
    fn test_product_reports_nested_field() {
        let mut product = Product {
            name: Name {
                name_label: "ethanol".to_string(),
                ..Default::default()
            },
            tags: Some(vec![Tag {
                tag_label: "   ".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        };
        assert_empty(product.sanitize_and_validate(), "tag_label");
    }
}
