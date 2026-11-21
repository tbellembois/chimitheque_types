#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::pubchem::Autocomplete;

    #[test]
    fn test_serde_round_trip() {
        let json = serde_json::json!({
            "total": 2,
            "dictionary_terms": {"compound": ["ethanol", "ethanolamine"]}
        });
        let autocomplete: Autocomplete = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(autocomplete.total, 2);
        assert_eq!(
            autocomplete.dictionary_terms.as_ref().unwrap().compound,
            ["ethanol", "ethanolamine"]
        );
        assert_eq!(serde_json::to_value(&autocomplete).unwrap(), json);
    }

    #[test]
    fn test_serde_omits_missing_terms() {
        let autocomplete: Autocomplete =
            serde_json::from_value(serde_json::json!({"total": 0})).unwrap();
        assert!(autocomplete.dictionary_terms.is_none());
        assert_eq!(
            serde_json::to_value(&autocomplete).unwrap(),
            serde_json::json!({"total": 0})
        );
    }
}
