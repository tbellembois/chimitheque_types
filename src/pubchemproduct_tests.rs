#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::error::PubchemParseError;
    use crate::producttype::ProductType;
    use crate::pubchemproduct::{PubchemProduct, query_string, query_strings};
    use serde_json::json;
    use std::error::Error as _;
    use std::fs::{self};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_pubchem_file_fields() {
        // Expected values checked against the raw JSON files.
        let files = [
            (
                include_str!("testdata/pubchem_files/pubchem_pug_view_Acetic Acid.json"),
                "H226 H303 H312 H314 H318 H334 H370 H371",
                "P210 P233 P240 P241 P242 P243 P260 P264 P264+P265 P270 P271 P280 P284 \
                 P301+P317 P301+P330+P331 P302+P352 P302+P361+P354 P303+P361+P353 \
                 P304+P340 P305+P354+P338 P308+P316 P316 P317 P321 P342+P316 P362+P364 \
                 P363 P370+P378 P403 P403+P235 P405 P501",
                "Danger",
                "GHS02 GHS05 GHS07 GHS08",
            ),
            (
                // Companies report both signal words.
                include_str!("testdata/pubchem_files/pubchem_pug_view_Acetaldehyde.json"),
                "H224 H302 H311 H315 H317 H319 H332 H335 H336 H341 H350 H351 H360 H361 \
                 H370 H372 H402",
                "P203 P210 P233 P240 P241 P242 P243 P260 P261 P262 P264 P264+P265 P270 \
                 P271 P272 P273 P280 P301+P317 P302+P352 P303+P361+P353 P304+P340 \
                 P305+P351+P338 P308+P316 P316 P317 P318 P319 P321 P330 P332+P317 \
                 P333+P317 P337+P317 P361+P364 P362+P364 P370+P378 P403+P233 P403+P235 \
                 P405 P501",
                "Danger Warning",
                "GHS02 GHS06 GHS07 GHS08",
            ),
            (
                // A combined hazard statement.
                include_str!(
                    "testdata/pubchem_files/pubchem_pug_view_1-Chloro-2,4-Dinitrobenzene.json"
                ),
                "H301 H301+H331 H302 H310 H311 H315 H317 H318 H331 H335 H341 H370 H372 \
                 H373 H400 H410",
                "P203 P260 P261 P262 P264 P264+P265 P270 P271 P272 P273 P280 P301+P316 \
                 P301+P317 P302+P352 P304+P340 P305+P354+P338 P308+P316 P316 P317 P318 \
                 P319 P321 P330 P332+P317 P333+P317 P361+P364 P362+P364 P391 P403+P233 \
                 P405 P501",
                "Danger",
                "GHS05 GHS06 GHS07 GHS08 GHS09",
            ),
            (
                // Not classified: no codes.
                include_str!("testdata/pubchem_files/pubchem_pug_view_4-Hydroxybenzaldehyde.json"),
                "",
                "",
                "",
                "",
            ),
        ];

        for (input, hazards, precautions, signals, symbols) in files {
            let product = PubchemProduct::from_pubchem_json(input).unwrap();
            let name = product.name.clone().unwrap_or_default();
            assert_codes(product.hs, hazards, &name);
            assert_codes(product.ps, precautions, &name);
            assert_codes(product.signal, signals, &name);
            assert_codes(product.symbols, symbols, &name);
        }
    }

    #[test]
    fn test_pubchem_file_identifiers() {
        let product = PubchemProduct::from_pubchem_json(include_str!(
            "testdata/pubchem_files/pubchem_pug_view_Acetic Acid.json"
        ))
        .unwrap();

        assert_eq!(product.name.as_deref(), Some("Acetic Acid"));
        assert_eq!(product.cas.as_deref(), Some("64-19-7"));
        assert_eq!(product.molecular_formula.as_deref(), Some("C2H4O2"));
        // PubChem names its canonical SMILES section "SMILES".
        assert_eq!(product.canonical_smiles.as_deref(), Some("CC(=O)O"));
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

    /// The `PubChem` files of `testdata/pubchem_files`, as (file name, JSON),
    /// sorted by file name.
    fn pubchem_files() -> Vec<(String, String)> {
        let mut files: Vec<(String, String)> = fs::read_dir("src/testdata/pubchem_files")
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                (name, fs::read_to_string(&path).unwrap())
            })
            .collect();
        files.sort();
        assert_ne!(files.len(), 0, "no PubChem files found");
        files
    }

    #[test]
    fn test_pubchem_files_pass_validation() {
        init_logger();

        // Collect every failure, to report them all at once.
        let mut failures = vec![];
        for (file, json) in pubchem_files() {
            let pubchem_product = match PubchemProduct::from_pubchem_json(&json) {
                Ok(pubchem_product) => pubchem_product,
                Err(error) => {
                    failures.push(format!("{file}: parse: {error}"));
                    continue;
                }
            };
            let mut product = match pubchem_product.to_product() {
                Ok(product) => product,
                Err(error) => {
                    failures.push(format!("{file}: to_product: {error}"));
                    continue;
                }
            };
            if let Err(error) = product.sanitize_and_validate() {
                failures.push(format!("{file}: validation: {error}"));
                continue;
            }

            let name = pubchem_product.name.clone().unwrap_or_default();
            assert_eq!(product.product_type, ProductType::Chem, "{file}");
            assert_eq!(product.name.name_label, name.to_uppercase(), "{file}");
            assert!(product.synonyms.is_none(), "{file}");
        }

        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn test_to_product_maps_fields() {
        let pubchem_product = PubchemProduct {
            name: Some("ethanol".to_string()),
            cas: Some("64-17-5".to_string()),
            ec: Some("200-578-6".to_string()),
            molecular_formula: Some("OC2H6".to_string()),
            inchi: Some("InChI=1S/C2H6O/c1-2-3/h3H,2H2,1H3".to_string()),
            inchi_key: Some("LFQSCWFLJHTTHZ-UHFFFAOYSA-N".to_string()),
            canonical_smiles: Some("CCO".to_string()),
            molecular_weight: Some(" 46.07 ".to_string()),
            molecular_weight_unit: Some("g/mol".to_string()),
            signal: Some(vec!["Danger".to_string(), "Warning".to_string()]),
            symbols: Some(vec!["GHS02".to_string(), "GHS07".to_string()]),
            hs: Some(vec!["H225".to_string()]),
            ps: Some(vec!["P210".to_string(), "P233".to_string()]),
            twodpicture: Some("iVBORw0KGgo=".to_string()),
            synonyms: Some(vec!["ethyl alcohol".to_string()]),
            ..Default::default()
        };

        let mut product = pubchem_product.to_product().unwrap();

        assert_eq!(
            product.cas_number.as_ref().unwrap().cas_number_label,
            "64-17-5"
        );
        assert_eq!(
            product.ce_number.as_ref().unwrap().ce_number_label,
            "200-578-6"
        );
        assert_eq!(product.product_molecular_weight, Some(46.07));
        assert_eq!(
            product.unit_molecular_weight.as_ref().unwrap().unit_label,
            "g/mol"
        );
        assert_eq!(
            product.signal_word.as_ref().unwrap().signal_word_label,
            "Danger"
        );
        let symbols: Vec<&str> = product
            .symbols
            .as_ref()
            .unwrap()
            .iter()
            .map(|symbol| symbol.symbol_label.as_str())
            .collect();
        assert_eq!(symbols, ["GHS02", "GHS07"]);
        assert_eq!(
            product.hazard_statements.as_ref().unwrap()[0].hazard_statement_reference,
            "H225"
        );
        assert_eq!(product.precautionary_statements.as_ref().unwrap().len(), 2);
        assert_eq!(
            product.product_twod_formula.as_deref(),
            Some("data:image/png;base64,iVBORw0KGgo=")
        );
        // Synonyms are not imported, and ids are left to be resolved.
        assert!(product.synonyms.is_none());
        assert!(
            product
                .unit_molecular_weight
                .as_ref()
                .unwrap()
                .unit_id
                .is_none()
        );

        // Validation cleans the new entries, e.g. sorts the formula.
        product.sanitize_and_validate().unwrap();
        assert_eq!(product.name.name_label, "ETHANOL");
        assert_eq!(
            product.empirical_formula.unwrap().empirical_formula_label,
            "C2H6O"
        );
    }

    #[test]
    fn test_to_product_keeps_the_most_severe_signal_word() {
        for (signals, expected) in [
            (vec!["Warning", "Danger"], "Danger"),
            (vec!["Warning"], "Warning"),
        ] {
            let pubchem_product = PubchemProduct {
                name: Some("ethanol".to_string()),
                signal: Some(signals.iter().map(ToString::to_string).collect()),
                ..Default::default()
            };
            let product = pubchem_product.to_product().unwrap();
            assert_eq!(product.signal_word.unwrap().signal_word_label, expected);
        }
    }

    #[test]
    fn test_to_product_rejects_invalid_molecular_weight() {
        let pubchem_product = PubchemProduct {
            name: Some("ethanol".to_string()),
            molecular_weight: Some("about 46".to_string()),
            ..Default::default()
        };
        let error = pubchem_product.to_product().unwrap_err();
        assert!(matches!(
            error,
            crate::error::ValidationError::InvalidFormat {
                field: "product_molecular_weight",
                ..
            }
        ));
    }
}
