#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::ParseError;
    use crate::sortorder::*;

    #[test]
    fn test_default_is_asc() {
        assert_eq!(SortOrder::default(), SortOrder::Asc);
    }

    #[test]
    fn test_string_representation() {
        for (order, expected) in [(SortOrder::Asc, "asc"), (SortOrder::Desc, "desc")] {
            assert_eq!(order.as_str(), expected);
            assert_eq!(order.to_string(), expected);
            assert_eq!(SortOrder::from_str(expected).unwrap(), order);
        }
    }

    #[test]
    fn test_from_str_ignores_case() {
        for input in ["ASC", "Asc", "aSc"] {
            assert_eq!(SortOrder::from_str(input).unwrap(), SortOrder::Asc);
        }
        for input in ["DESC", "Desc", "dEsC"] {
            assert_eq!(SortOrder::from_str(input).unwrap(), SortOrder::Desc);
        }
    }

    #[test]
    fn test_from_str_rejects_invalid() {
        for input in ["", " asc", "desc ", "ascending", "up", "1"] {
            let error = SortOrder::from_str(input).unwrap_err();
            assert!(
                matches!(&error, ParseError::ParseSortOrderError(value) if value == input),
                "{input:?}"
            );
            assert_eq!(error.to_string(), format!("wrong sort order: {input}"));
        }
    }

    #[test]
    fn test_serde_uses_lowercase_strings() {
        for (order, expected) in [(SortOrder::Asc, "asc"), (SortOrder::Desc, "desc")] {
            assert_eq!(
                serde_json::to_value(order).unwrap(),
                serde_json::json!(expected)
            );
        }
    }

    #[test]
    fn test_serde_deserialize_ignores_case() {
        for (input, expected) in [
            ("asc", SortOrder::Asc),
            ("ASC", SortOrder::Asc),
            ("desc", SortOrder::Desc),
            ("Desc", SortOrder::Desc),
        ] {
            let parsed: SortOrder = serde_json::from_value(serde_json::json!(input)).unwrap();
            assert_eq!(parsed, expected);
            // Output is always lowercase.
            assert_eq!(
                serde_json::to_value(parsed).unwrap(),
                serde_json::json!(expected.as_str())
            );
        }
    }

    #[test]
    fn test_serde_rejects_invalid() {
        for input in [
            serde_json::json!(""),
            serde_json::json!("up"),
            serde_json::json!(null),
            serde_json::json!(1),
        ] {
            assert!(
                serde_json::from_value::<SortOrder>(input.clone()).is_err(),
                "{input}"
            );
        }
    }
}
