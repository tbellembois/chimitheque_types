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
    use serde_json::json;

    const TYPES: [(ProductType, &str); 3] = [
        (ProductType::Chem, "chem"),
        (ProductType::Bio, "bio"),
        (ProductType::Cons, "cons"),
    ];

    const INVALID: [&str; 5] = ["", "Chem", "CHEM", "chemical", "consumable"];

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        for (product_type, text) in TYPES {
            assert_eq!(serde_json::to_value(product_type).unwrap(), json!(text));
            assert_eq!(
                serde_json::from_value::<ProductType>(json!(text)).unwrap(),
                product_type
            );
        }
    }

    #[test]
    fn test_serde_rejects_invalid() {
        for text in INVALID {
            assert!(serde_json::from_value::<ProductType>(json!(text)).is_err());
        }
    }

    // Other functions.

    #[test]
    fn test_default() {
        assert_eq!(ProductType::default(), ProductType::Chem);
    }

    #[test]
    fn test_display_and_from_str() {
        for (product_type, text) in TYPES {
            assert_eq!(product_type.to_string(), text);
            assert_eq!(ProductType::from_str(text).unwrap(), product_type);
        }
    }

    #[test]
    fn test_from_str_rejects_invalid() {
        for text in INVALID {
            let error = ProductType::from_str(text).unwrap_err();
            assert!(
                matches!(&error, ParseError::ParseProductTypeError(value) if value == text),
                "{text:?}"
            );
            assert_eq!(error.to_string(), format!("wrong product type: {text}"));
        }
    }
}
