#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::pubchemproduct::PubchemProduct;
    use serde_json::json;
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
            let product = PubchemProduct::from_pubchem_json(input);
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
        PubchemProduct::from_pubchem_json(&input.to_string())
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
            PubchemProduct::from_pubchem_json(r#"{"Record":{"RecordTitle":"H302 P261"}}"#);
        assert_codes(product.hs, "", "missing hazards");
        assert_codes(product.ps, "", "missing precautions");
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        for entry in fs::read_dir("src/testdata/").unwrap() {
            let entry_path = entry.unwrap().path();
            let json_string =
                fs::read_to_string(entry_path).expect("error while opening json file");
            let product = PubchemProduct::from_pubchem_json(json_string.as_str());

            assert!(product.iupac_name.is_some());
        }
    }
}
