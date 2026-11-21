#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::{
        classofcompound::ClassOfCompound, error::ValidationError, name::Name, producer::Producer,
        producerref::ProducerRef, product::*, supplier::Supplier, supplierref::SupplierRef,
    };

    /// Makes a value invalid, and the field expected in the error.
    type InvalidCase<T> = (fn(&mut T), &'static str);

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

    #[test]
    fn test_sanitize_and_validate_optional_text() {
        let mut product = product_named("ethanol");
        product.product_inchikey = Some("  LFQSCWFLJHTTHZ-UHFFFAOYSA-N ".to_string());
        product.product_specificity = Some("   ".to_string());
        product.product_remark = Some("  first line\n  second line  ".to_string());
        product.product_msds = Some(String::new());

        product.sanitize_and_validate().unwrap();

        assert_eq!(
            product.product_inchikey.as_deref(),
            Some("LFQSCWFLJHTTHZ-UHFFFAOYSA-N")
        );
        assert_eq!(product.product_specificity, None);
        // Free text keeps its line breaks.
        assert_eq!(
            product.product_remark.as_deref(),
            Some("first line\n  second line")
        );
        assert_eq!(product.product_msds, None);
    }

    #[test]
    fn test_sanitize_and_validate_numbers() {
        let mut product = product_named("ethanol");
        product.product_molecular_weight = Some(46.07);
        product.product_temperature = Some(-20.0);
        product.product_number_per_carton = Some(12);
        product.product_number_per_bag = Some(1);
        product.sanitize_and_validate().unwrap();

        let cases: [InvalidCase<Product>; 5] = [
            (
                |p| p.product_molecular_weight = Some(0.0),
                "product_molecular_weight",
            ),
            (
                |p| p.product_molecular_weight = Some(f64::NAN),
                "product_molecular_weight",
            ),
            (
                |p| p.product_temperature = Some(f64::INFINITY),
                "product_temperature",
            ),
            (
                |p| p.product_number_per_carton = Some(0),
                "product_number_per_carton",
            ),
            (
                |p| p.product_number_per_bag = Some(-1),
                "product_number_per_bag",
            ),
        ];
        for (set_invalid, expected_field) in cases {
            let mut product = product_named("ethanol");
            set_invalid(&mut product);
            let error = product.sanitize_and_validate().unwrap_err();
            assert!(
                matches!(error, ValidationError::OutOfRange { field, .. } if field == expected_field),
                "{error:?}"
            );
        }
    }

    #[test]
    fn test_sanitize_and_validate_new_references() {
        let mut product = product_named("ethanol");
        product.producer_ref = Some(ProducerRef {
            producer_ref_label: "  REF  1 ".to_string(),
            producer: Producer {
                producer_id: Some(1),
                ..Default::default()
            },
            ..Default::default()
        });
        product.supplier_refs = Some(vec![SupplierRef {
            supplier_ref_label: " S-REF ".to_string(),
            supplier: Supplier {
                supplier_id: Some(1),
                ..Default::default()
            },
            ..Default::default()
        }]);
        product.sanitize_and_validate().unwrap();
        assert_eq!(product.producer_ref.unwrap().producer_ref_label, "REF 1");
        assert_eq!(
            product.supplier_refs.unwrap()[0].supplier_ref_label,
            "S-REF"
        );

        let mut product = product_named("ethanol");
        product.producer_ref = Some(ProducerRef::default());
        assert_empty_field(&mut product, "producer_ref_label");

        let mut product = product_named("ethanol");
        product.supplier_refs = Some(vec![SupplierRef::default()]);
        assert_empty_field(&mut product, "supplier_ref_label");

        // Existing references (with an id) are left unchanged.
        let mut product = product_named("ethanol");
        product.producer_ref = Some(ProducerRef {
            producer_ref_id: Some(5),
            ..Default::default()
        });
        product.sanitize_and_validate().unwrap();
    }
}
