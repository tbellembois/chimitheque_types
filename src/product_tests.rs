#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::{classofcompound::ClassOfCompound, error::ValidationError, name::Name, product::*};

    fn product_named(label: &str) -> Product {
        Product {
            name: Name {
                name_label: label.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn new_synonym(label: &str) -> Name {
        Name {
            name_label: label.to_string(),
            ..Default::default()
        }
    }

    fn new_class(label: &str) -> ClassOfCompound {
        ClassOfCompound {
            class_of_compound_label: label.to_string(),
            ..Default::default()
        }
    }

    fn assert_empty_field(product: &mut Product, expected_field: &str) {
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(
            matches!(error, ValidationError::Empty { field } if field == expected_field),
            "{error:?}"
        );
    }

    #[test]
    fn test_sanitize_and_validate_cleans_new_synonyms() {
        let mut product = product_named("ethanol");
        product.synonyms = Some(vec![new_synonym("  ethyl   alcohol "), new_synonym("etoh")]);

        product.sanitize_and_validate().unwrap();

        let synonyms: Vec<&str> = product
            .synonyms
            .as_ref()
            .unwrap()
            .iter()
            .map(|synonym| synonym.name_label.as_str())
            .collect();
        assert_eq!(synonyms, ["ETHYL ALCOHOL", "ETOH"]);
    }

    #[test]
    fn test_sanitize_and_validate_rejects_empty_new_synonym() {
        let mut product = product_named("ethanol");
        product.synonyms = Some(vec![new_synonym("etoh"), new_synonym("   ")]);
        assert_empty_field(&mut product, "name_label");
    }

    #[test]
    fn test_sanitize_and_validate_keeps_existing_synonyms() {
        let mut product = product_named("ethanol");
        product.synonyms = Some(vec![Name {
            name_id: Some(7),
            name_label: "  stored   as is ".to_string(),
            ..Default::default()
        }]);

        product.sanitize_and_validate().unwrap();

        assert_eq!(product.synonyms.unwrap()[0].name_label, "  stored   as is ");
    }

    #[test]
    fn test_sanitize_and_validate_cleans_new_classes_of_compound() {
        let mut product = product_named("ethanol");
        product.classes_of_compound = Some(vec![new_class("  primary   alcohol ")]);

        product.sanitize_and_validate().unwrap();

        assert_eq!(
            product.classes_of_compound.unwrap()[0].class_of_compound_label,
            "primary alcohol"
        );
    }

    #[test]
    fn test_sanitize_and_validate_rejects_empty_new_class_of_compound() {
        let mut product = product_named("ethanol");
        product.classes_of_compound = Some(vec![new_class("")]);
        assert_empty_field(&mut product, "class_of_compound_label");
    }

    #[test]
    fn test_sanitize_and_validate_keeps_existing_classes_of_compound() {
        let mut product = product_named("ethanol");
        product.classes_of_compound = Some(vec![ClassOfCompound {
            class_of_compound_id: Some(3),
            class_of_compound_label: String::new(),
            ..Default::default()
        }]);

        product.sanitize_and_validate().unwrap();

        assert_eq!(
            product.classes_of_compound.unwrap()[0].class_of_compound_label,
            ""
        );
    }
}
