#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::test_helpers::{assert_serde_rejects_missing, assert_value_round_trip};
    use crate::{
        casnumber::CasNumber, category::Category, classofcompound::ClassOfCompound,
        error::ValidationError, linearformula::LinearFormula, name::Name, producer::Producer,
        producerref::ProducerRef, product::*, producttype::ProductType, supplier::Supplier,
        supplierref::SupplierRef, tag::Tag, unit::Unit,
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

    // Serialization and deserialization.

    fn serde_sample() -> Product {
        Product {
            product_id: Some(1),
            product_type: ProductType::Bio,
            product_restricted: true,
            product_remark: Some("first line\nsecond line".to_string()),
            product_molecular_weight: Some(46.07),
            unit_molecular_weight: Some(Unit {
                unit_id: Some(3),
                unit_label: "g/mol".to_string(),
                unit_multiplier: 1.0,
                ..Default::default()
            }),
            name: Name {
                name_id: Some(2),
                name_label: "ETHANOL".to_string(),
                ..Default::default()
            },
            cas_number: Some(CasNumber {
                cas_number_id: Some(4),
                cas_number_label: "64-17-5".to_string(),
                ..Default::default()
            }),
            synonyms: Some(vec![new_synonym("ETHYL ALCOHOL")]),
            tags: Some(vec![Tag {
                tag_id: Some(5),
                tag_label: "solvent".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        }
    }

    #[test]
    fn test_serde() {
        let json = assert_value_round_trip(&serde_sample());
        assert_eq!(json["product_type"], "bio");
        assert_eq!(json["product_restricted"], true);
        assert_eq!(json["product_remark"], "first line\nsecond line");
        assert_eq!(json["unit_molecular_weight"]["unit_label"], "g/mol");
        assert_eq!(json["name"]["name_label"], "ETHANOL");
        assert_eq!(json["cas_number"]["cas_number_label"], "64-17-5");
        assert_eq!(json["synonyms"][0]["name_label"], "ETHYL ALCOHOL");
        assert_eq!(json["tags"][0]["tag_label"], "solvent");
    }

    #[test]
    fn test_serde_defaults() {
        let value: Product = serde_json::from_value(serde_json::json!({
            "product_type": "chem",
            "name": {"name_label": "ETHANOL"}
        }))
        .unwrap();
        assert_eq!(value.product_id, None);
        assert!(!value.product_restricted);
        assert!(!value.product_radioactive);
        assert!(!value.product_has_bookmark);
        assert_eq!(value.person.person_id, None);
        assert!(value.cas_number.is_none());
        assert!(value.synonyms.is_none());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        let json = assert_value_round_trip(&serde_sample());
        assert_serde_rejects_missing::<Product>(&json, &["product_type", "name"]);
    }

    // sanitize_and_validate.

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
            Some("first line\nsecond line")
        );
        assert_eq!(product.product_msds, None);
    }

    #[test]
    fn test_sanitize_and_validate_numbers() {
        let mut product = product_named("ethanol");
        product.product_molecular_weight = Some(46.07);
        product.product_temperature = Some(-20.0);
        product.product_concentration = Some(0.0);
        product.product_number_per_carton = Some(12);
        product.product_number_per_bag = Some(1);
        product.unit_molecular_weight = Some(Unit::default());
        product.unit_temperature = Some(Unit::default());
        product.unit_concentration = Some(Unit::default());
        product.sanitize_and_validate().unwrap();

        let cases: [InvalidCase<Product>; 7] = [
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
                |p| p.product_concentration = Some(-0.1),
                "product_concentration",
            ),
            (
                |p| p.product_concentration = Some(f64::NAN),
                "product_concentration",
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

    #[test]
    fn test_sanitize_and_validate_new_category_and_linear_formula() {
        // A new category and a new linear formula (without id) are cleaned.
        let mut product = product_named("ethanol");
        product.category = Some(Category {
            category_label: "  organic   solvents ".to_string(),
            ..Default::default()
        });
        product.linear_formula = Some(LinearFormula {
            linear_formula_label: " CH3CH2OH ".to_string(),
            ..Default::default()
        });
        product.sanitize_and_validate().unwrap();
        assert_eq!(product.category.unwrap().category_label, "organic solvents");
        assert_eq!(
            product.linear_formula.unwrap().linear_formula_label,
            "CH3CH2OH"
        );

        // They are rejected when invalid.
        let mut product = product_named("ethanol");
        product.category = Some(Category::default());
        assert_empty_field(&mut product, "category_label");

        let mut product = product_named("ethanol");
        product.linear_formula = Some(LinearFormula {
            linear_formula_label: "X123X".to_string(),
            ..Default::default()
        });
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(
            matches!(
                error,
                ValidationError::InvalidFormula {
                    field: "linear_formula_label",
                    ..
                }
            ),
            "{error:?}"
        );

        // Existing ones (with an id) are not validated.
        let mut product = product_named("ethanol");
        product.category = Some(Category {
            category_id: Some(1),
            ..Default::default()
        });
        product.linear_formula = Some(LinearFormula {
            linear_formula_id: Some(2),
            ..Default::default()
        });
        product.sanitize_and_validate().unwrap();
    }

    #[test]
    fn test_sanitize_and_validate_identifiers() {
        let mut product = product_named("ethanol");
        product.product_inchi = Some(" InChI=1S/C2H6O/c1-2-3/h3H,2H2,1H3 ".to_string());
        product.product_inchikey = Some("LFQSCWFLJHTTHZ-UHFFFAOYSA-N".to_string());
        product.sanitize_and_validate().unwrap();
        assert_eq!(
            product.product_inchi.as_deref(),
            Some("InChI=1S/C2H6O/c1-2-3/h3H,2H2,1H3")
        );

        let mut product = product_named("ethanol");
        product.product_inchi = Some("C2H6O".to_string());
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidFormat {
                field: "product_inchi",
                ..
            }
        ));
        assert_eq!(error.to_string(), "product_inchi: must be a valid InChI");

        let mut product = product_named("ethanol");
        product.product_canonical_smiles = Some(" CCO ".to_string());
        product.sanitize_and_validate().unwrap();
        assert_eq!(product.product_canonical_smiles.as_deref(), Some("CCO"));

        let mut product = product_named("ethanol");
        product.product_canonical_smiles = Some("C1CC".to_string());
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidFormat {
                field: "product_canonical_smiles",
                ..
            }
        ));

        let mut product = product_named("ethanol");
        product.product_inchikey = Some("LFQSCWFLJHTTHZ".to_string());
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidFormat {
                field: "product_inchikey",
                ..
            }
        ));
    }

    #[test]
    fn test_sanitize_and_validate_units_required() {
        let mut product = product_named("ethanol");
        product.product_molecular_weight = Some(46.07);
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::Required {
                field: "unit_molecular_weight",
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "unit_molecular_weight: required when product_molecular_weight is set"
        );

        let mut product = product_named("ethanol");
        product.product_temperature = Some(20.0);
        let error = product.sanitize_and_validate().unwrap_err();
        assert!(matches!(
            error,
            ValidationError::Required {
                field: "unit_temperature",
                ..
            }
        ));

        // A unit without a value is accepted.
        let mut product = product_named("ethanol");
        product.unit_temperature = Some(Unit::default());
        product.sanitize_and_validate().unwrap();

        let mut product = product_named("ethanol");
        product.product_concentration = Some(1.0);
        assert_eq!(
            product.sanitize_and_validate().unwrap_err().to_string(),
            "unit_concentration: required when product_concentration is set"
        );
    }

    /// Unit types as in the base data: °C, g/mol, mL and mg/L.
    fn base_unit_type(unit_id: u64) -> Option<crate::unittype::UnitType> {
        use crate::unittype::UnitType;
        match unit_id {
            14 => Some(UnitType::Temperature),
            24 => Some(UnitType::MolecularWeight),
            2 => Some(UnitType::Quantity),
            21 => Some(UnitType::Concentration),
            _ => None,
        }
    }

    fn unit_with_id(unit_id: Option<u64>) -> Unit {
        // The request may carry only the id: unit_type keeps its default value.
        Unit {
            unit_id,
            ..Default::default()
        }
    }

    // Other functions.

    #[test]
    fn test_validate_unit_types() {
        let mut product = product_named("ethanol");
        product.validate_unit_types(base_unit_type).unwrap();

        product.unit_temperature = Some(unit_with_id(Some(14)));
        product.unit_molecular_weight = Some(unit_with_id(Some(24)));
        product.unit_concentration = Some(unit_with_id(Some(21)));
        product.validate_unit_types(base_unit_type).unwrap();

        // mL (a quantity unit) as concentration unit.
        product.unit_concentration = Some(unit_with_id(Some(2)));
        assert_eq!(
            product
                .validate_unit_types(base_unit_type)
                .unwrap_err()
                .to_string(),
            "unit_concentration: must be a concentration unit"
        );
        product.unit_concentration = None;

        // mL (a quantity unit) as temperature unit.
        product.unit_temperature = Some(unit_with_id(Some(2)));
        let error = product.validate_unit_types(base_unit_type).unwrap_err();
        assert!(matches!(
            error,
            ValidationError::InvalidUnit {
                field: "unit_temperature",
                ..
            }
        ));
        assert_eq!(
            error.to_string(),
            "unit_temperature: must be a temperature unit"
        );

        // °C as molecular weight unit, an unknown id, and a unit without id.
        for unit_id in [Some(14), Some(999), None] {
            let mut product = product_named("ethanol");
            product.unit_molecular_weight = Some(unit_with_id(unit_id));
            let error = product.validate_unit_types(base_unit_type).unwrap_err();
            assert!(
                matches!(
                    error,
                    ValidationError::InvalidUnit {
                        field: "unit_molecular_weight",
                        ..
                    }
                ),
                "{unit_id:?}"
            );
        }
    }
}
