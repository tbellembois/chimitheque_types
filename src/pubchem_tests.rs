#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::pubchem::{Autocomplete, AutocompleteTerm};
    use crate::test_helpers::{assert_serde_rejects_missing, assert_serde_round_trip};
    use serde_json::json;

    fn sample() -> serde_json::Value {
        json!({
            "total": 2,
            "dictionary_terms": {"compound": ["ethanol", "ethanolamine"]}
        })
    }

    // Serialization and deserialization.

    #[test]
    fn test_serde() {
        let value = assert_serde_round_trip::<Autocomplete>(&sample());
        assert_eq!(value.total, 2);
        assert_eq!(
            value.dictionary_terms.unwrap().compound,
            ["ethanol", "ethanolamine"]
        );
    }

    #[test]
    fn test_serde_optional_fields() {
        // Missing dictionary terms are deserialized as None and not serialized.
        let value = assert_serde_round_trip::<Autocomplete>(&json!({"total": 0}));
        assert!(value.dictionary_terms.is_none());
    }

    #[test]
    fn test_serde_rejects_missing_required_fields() {
        assert_serde_rejects_missing::<Autocomplete>(&sample(), &["total"]);
        assert_serde_rejects_missing::<AutocompleteTerm>(
            &json!({"compound": ["ethanol"]}),
            &["compound"],
        );
    }
}
