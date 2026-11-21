#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use std::str::FromStr;

    use crate::error::ParseError;
    use crate::producttype::ProductType;

    #[test]
    fn test_default_is_chem() {
        assert_eq!(ProductType::default(), ProductType::Chem);
    }

    #[test]
    fn test_string_and_serde_representation() {
        for (product_type, text) in [
            (ProductType::Chem, "chem"),
            (ProductType::Bio, "bio"),
            (ProductType::Cons, "cons"),
        ] {
            assert_eq!(product_type.to_string(), text);
            assert_eq!(ProductType::from_str(text).unwrap(), product_type);
            assert_eq!(
                serde_json::to_value(product_type).unwrap(),
                serde_json::json!(text)
            );
            let parsed: ProductType = serde_json::from_value(serde_json::json!(text)).unwrap();
            assert_eq!(parsed, product_type);
        }
    }

    #[test]
    fn test_rejects_unknown_values() {
        for text in ["", "Chem", "CHEM", "chemical", "consumable"] {
            let error = ProductType::from_str(text).unwrap_err();
            assert!(
                matches!(&error, ParseError::ParseProductTypeError(value) if value == text),
                "{text:?}"
            );
            assert_eq!(error.to_string(), format!("wrong product type: {text}"));
            assert!(serde_json::from_value::<ProductType>(serde_json::json!(text)).is_err());
        }
    }
}
