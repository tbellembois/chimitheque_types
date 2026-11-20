#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::PubchemParseError;
    use crate::pubchemproduct::{PubchemProduct, query_string, query_strings};
    use serde_json::json;
    use std::error::Error as _;
    use std::fs::{self};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_fixture_safety_codes() {
        let fixtures = [
            (
                "butanediol",
                include_str!("testdata/pubchem_pug_view_1.4-butanediol.json"),
                "H302 H336",
                "P261 P264 P270 P271 P301+P317 P304+P340 P319 P330 P403+P233 P405 P501",
            ),
            (
                "acetaminophen",
                include_str!("testdata/pubchem_pug_view_acetaminophen.json"),
                "H302 H315 H319 H335 H341 H370 H371 H372 H373 H401 H411 H412",
                "P203 P260 P261 P264 P264+P265 P270 P271 P273 P280 P301+P317 P302+P352 \
                 P304+P340 P305+P351+P338 P308+P316 P318 P319 P321 P330 P332+P317 \
                 P337+P317 P362+P364 P391 P403+P233 P405 P501",
            ),
            (
                "acetic acid",
                include_str!("testdata/pubchem_pug_view_acetic_acid.json"),
                "H226 H314",
                "P210 P233 P240 P241 P242 P243 P260 P264 P280 P301+P330+P331 \
                 P302+P361+P354 P303+P361+P353 P304+P340 P305+P354+P338 P316 P321 P363 \
                 P370+P378 P403+P235 P405 P501",
            ),
            (
                "aspirin",
                include_str!("testdata/pubchem_pug_view_aspirin.json"),
                "H302 H315 H316 H319 H334 H335 H360 H370 H371 H372 H373 H412",
                "P203 P233 P260 P261 P264 P264+P265 P270 P271 P273 P280 P284 P301+P317 \
                 P302+P352 P304+P340 P305+P351+P338 P308+P316 P318 P319 P321 P330 \
                 P332+P317 P337+P317 P342+P316 P362+P364 P403 P403+P233 P405 P501",
            ),
            (
                "carbonate",
                include_str!("testdata/pubchem_pug_view_carbonate.json"),
                "",
                "",
            ),
            (
                "vanadium pentoxide",
                include_str!("testdata/pubchem_pug_view_vanadium_pentoxide.json"),
                "H300 H301 H302 H318 H319 H330 H332 H335 H340 H341 H350 H351 H361 \
                 H361fd H362 H370 H372 H373 H401 H410 H411",
                "P203 P260 P261 P263 P264 P264+P265 P270 P271 P273 P280 P284 P301+P316 \
                 P301+P317 P304+P340 P305+P351+P338 P305+P354+P338 P308+P316 P316 P317 \
                 P318 P319 P320 P321 P330 P337+P317 P391 P403+P233 P405 P501",
            ),
        ];

        for (name, input, hazards, precautions) in fixtures {
            let product = PubchemProduct::from_pubchem_json(input)
                .unwrap_or_else(|error| panic!("{name}: {error}"));
            assert_codes(product.hs, hazards, name);
            assert_codes(product.ps, precautions, name);
        }
    }

    fn assert_codes(actual: Option<Vec<String>>, expected: &str, context: &str) {
        let actual = actual.unwrap_or_default();
        let expected: Vec<&str> = expected.split_whitespace().collect();
        assert_eq!(actual, expected, "{context}");
    }

    fn safety_product(hazards: &[&str], precautions: &[&str]) -> PubchemProduct {
        let input = json!({
            "Record": {
                "Section": [{
                    "TOCHeading": "Safety and Hazards",
                    "Information": [
                        {
                            "Name": "GHS Hazard Statements",
                            "Value": {
                                "StringWithMarkup": hazards.iter()
                                    .map(|text| json!({"String": text}))
                                    .collect::<Vec<_>>()
                            }
                        },
                        {
                            "Name": "Precautionary Statement Codes",
                            "Value": {
                                "StringWithMarkup": precautions.iter()
                                    .map(|text| json!({"String": text}))
                                    .collect::<Vec<_>>()
                            }
                        }
                    ]
                }]
            }
        });
        PubchemProduct::from_pubchem_json(&input.to_string()).unwrap()
    }

    #[test]
    fn test_safety_codes_preserve_combinations_and_suffixes() {
        let product = safety_product(
            &[
                "H300+H310+H330: Fatal if swallowed, in contact with skin or if inhaled",
                "H360FD (50%): May damage fertility and the unborn child",
                "H361fd: Suspected of damaging fertility and the unborn child",
                "H350i: May cause cancer by inhalation",
                "EUH066: Repeated exposure may cause skin dryness or cracking",
            ],
            &["P301+P310, P305+P351+P338, and P501"],
        );

        assert_codes(
            product.hs,
            "EUH066 H300+H310+H330 H350i H360FD H361fd",
            "combined and suffixed hazards",
        );
        assert_codes(
            product.ps,
            "P301+P310 P305+P351+P338 P501",
            "combined precautions",
        );
    }

    #[test]
    fn test_safety_codes_collect_all_strings_sort_and_deduplicate() {
        let product = safety_product(
            &[
                "H336: May cause drowsiness",
                "H302 (89.6%): Harmful if swallowed",
                "H302",
            ],
            &["P501, P261", "P301 + P310, P261", "P301+P310"],
        );

        assert_codes(product.hs, "H302 H336", "deduplicated hazards");
        assert_codes(
            product.ps,
            "P261 P301+P310 P501",
            "deduplicated precautions",
        );
    }

    #[test]
    fn test_safety_codes_do_not_match_partial_identifiers_or_other_fields() {
        let product = safety_product(
            &["XH302 H3020 H36 H361fdd H302_suffix P261"],
            &["XP261 P2610 P26 P261_suffix H302"],
        );
        assert_codes(product.hs, "", "invalid hazards");
        assert_codes(product.ps, "", "invalid precautions");

        let product =
            PubchemProduct::from_pubchem_json(r#"{"Record":{"RecordTitle":"H302 P261"}}"#).unwrap();
        assert_codes(product.hs, "", "missing hazards");
        assert_codes(product.ps, "", "missing precautions");
    }

    #[test]
    fn test_invalid_json_preserves_source() {
        for input in ["", "{", r#"{"Record":}"#] {
            let error = PubchemProduct::from_pubchem_json(input).unwrap_err();
            assert!(matches!(&error, PubchemParseError::InvalidJson(_)));
            let source = error
                .source()
                .unwrap()
                .downcast_ref::<serde_json::Error>()
                .unwrap();
            assert!(source.is_syntax() || source.is_eof());
            assert!(error.to_string().contains("invalid PubChem JSON"));
        }
    }

    #[test]
    fn test_invalid_document_envelope() {
        for (input, field, actual) in [
            ("null", "$", "null"),
            ("[]", "$", "array"),
            ("42", "$", "number"),
            ("{}", "$.Record", "missing"),
            (
                r#"{"Fault":{"Message":"Not found"}}"#,
                "$.Record",
                "missing",
            ),
            (r#"{"Record":null}"#, "$.Record", "null"),
            (r#"{"Record":[]}"#, "$.Record", "array"),
            (r#"{"Record":"water"}"#, "$.Record", "string"),
        ] {
            let error = PubchemProduct::from_pubchem_json(input).unwrap_err();
            assert!(
                matches!(&error, PubchemParseError::InvalidType {
                    field: found_field, expected: "object", actual: found_type,
                } if *found_field == field && *found_type == actual),
                "{input}: {error}"
            );
            assert!(error.to_string().contains(field));
            assert!(error.source().is_none());
        }
    }

    #[test]
    fn test_missing_optional_fields_are_not_errors() {
        let product = PubchemProduct::from_pubchem_json(r#"{"Record":{}}"#).unwrap();
        assert_eq!(serde_json::to_value(product).unwrap(), json!({}));

        let product =
            PubchemProduct::from_pubchem_json(r#"{"Record":{"RecordTitle":"Water"}}"#).unwrap();
        assert_eq!(product.name.as_deref(), Some("Water"));
        assert!(product.cas.is_none());
        assert!(product.hs.is_none());
        assert!(product.ps.is_none());
    }

    #[test]
    fn test_non_string_scalar_fields_are_errors() {
        // A matched null is malformed data, not the JSONPath no-match sentinel.
        for (value, actual) in [
            (json!(null), "null"),
            (json!(42), "number"),
            (json!(true), "boolean"),
            (json!([]), "array"),
            (json!({}), "object"),
        ] {
            let input = json!({"Record": {"RecordTitle": value}});
            let error = PubchemProduct::from_pubchem_json(&input.to_string()).unwrap_err();
            assert!(matches!(error, PubchemParseError::InvalidType {
                field: "name", expected: "string", actual: found_type,
            } if found_type == actual));
        }
    }

    #[test]
    fn test_non_string_list_entries_are_not_silently_dropped() {
        for (name, field) in [
            ("GHS Hazard Statements", "hs"),
            ("Precautionary Statement Codes", "ps"),
            ("Signal", "signal"),
        ] {
            let input = json!({
                "Record": {
                    "Information": [{
                        "Name": name,
                        "Value": {"StringWithMarkup": [
                            {"String": "H302 P261 Warning"}, {"String": 42}
                        ]}
                    }]
                }
            });
            let error = PubchemProduct::from_pubchem_json(&input.to_string()).unwrap_err();
            assert!(matches!(error, PubchemParseError::InvalidType {
                field: found_field, expected: "string", actual: "number",
            } if found_field == field));
        }
    }

    #[test]
    fn test_query_errors_include_field_and_query() {
        let query = "$[";
        let error = query_strings(&json!({}), "name", query).unwrap_err();
        assert!(matches!(&error, PubchemParseError::Query {
            field: "name", query: found_query, message,
        } if *found_query == query && !message.is_empty()));
        assert!(error.to_string().contains("name"));
        assert!(error.to_string().contains(query));
    }

    #[test]
    fn test_scalar_selection_preserves_first_match_but_validates_all_matches() {
        let input = json!({"values": ["first", "second"]});
        assert_eq!(
            query_string(&input, "name", "$.values[*]")
                .unwrap()
                .as_deref(),
            Some("first")
        );

        let input = json!({"values": ["first", null]});
        assert!(matches!(
            query_string(&input, "name", "$.values[*]").unwrap_err(),
            PubchemParseError::InvalidType {
                field: "name",
                expected: "string",
                actual: "null",
            }
        ));
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        for entry in fs::read_dir("src/testdata/").unwrap() {
            let entry_path = entry.unwrap().path();
            let json_string =
                fs::read_to_string(entry_path).expect("error while opening json file");
            let product = PubchemProduct::from_pubchem_json(json_string.as_str()).unwrap();

            assert!(product.iupac_name.is_some());
        }
    }
}
