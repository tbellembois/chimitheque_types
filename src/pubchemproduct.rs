use chimitheque_defines::SYMBOL_RE;
use jsonpath_rust::JsonPathQuery;
use log::{debug, trace};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::LazyLock;

// PubChem embeds codes in prose, unlike the shared tab-separated reference-table regexes.
#[allow(
    clippy::expect_used,
    reason = "the regex is a constant covered by extraction tests"
)]
static PUBCHEM_HAZARD_CODE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?-u)\b(?:EU)?H[0-9]{3}[A-Za-z]{0,2}(?:[ \t]*\+[ \t]*(?:EU)?H[0-9]{3}[A-Za-z]{0,2})*\b",
    )
    .expect("valid PubChem hazard code regex")
});

#[allow(
    clippy::expect_used,
    reason = "the regex is a constant covered by extraction tests"
)]
static PUBCHEM_PRECAUTIONARY_CODE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?-u)\bP[0-9]{3}(?:[ \t]*\+[ \t]*P[0-9]{3})*\b")
        .expect("valid PubChem precautionary code regex")
});

fn extract_safety_codes(values: &[String], pattern: &Regex) -> Vec<String> {
    let mut codes: Vec<String> = values
        .iter()
        .flat_map(|value| pattern.find_iter(value))
        .map(|code| code.as_str().replace([' ', '\t'], ""))
        .collect();
    codes.sort();
    codes.dedup();
    codes
}

/// Helper function to safely extract a String value from a JSON Value
/// Returns None if the value doesn't exist or can't be converted to a string
#[must_use]
fn extract_string(value: &Value) -> Option<String> {
    debug!("extract_string called");
    value
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|first| first.as_str())
        .map(std::string::ToString::to_string)
}

/// Helper function to safely extract a Vec<String> from a JSON Value
/// Returns Some(Vec<String>) if extraction succeeds, None otherwise
#[must_use]
fn extract_string_vec(value: &Value) -> Option<Vec<String>> {
    value.as_array().map(|arr| {
        arr.iter()
            .filter_map(|item| item.as_str().map(std::string::ToString::to_string))
            .collect()
    })
}

/// Helper function to safely query and extract a field from JSON
/// Returns None if the field doesn't exist, otherwise Some(value)
#[must_use]
fn safe_json_query(json: &Value, query: &str) -> Option<Value> {
    json.clone().path(query).ok()
}

// A simplified pubchem product representation.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PubchemProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iupac_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inchi: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inchi_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_smiles: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_formula: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weight_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boiling_point: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twodpicture: Option<String>, // base64 encoded png
}

impl PubchemProduct {
    #[must_use]
    pub fn from_pubchem_json(json_content: &str) -> PubchemProduct {
        // Final result.
        let mut product = PubchemProduct {
            ..Default::default()
        };

        // Parse JSON safely, return empty product on failure
        // No panic on malformed JSON - return default product
        let json_content: Value = match serde_json::from_str(json_content) {
            Ok(v) => v,
            Err(_) => return product,
        };

        // Name - safely extract if field exists
        if let Some(name_value) = safe_json_query(&json_content, "$.Record.RecordTitle")
            && let Some(name_str) = extract_string(&name_value)
        {
            debug!("name: {name_str}");
            product.name = Some(name_str);
        }

        // IUPAC name - safely extract if field exists
        if let Some(iupac_name_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='IUPAC Name')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(iupac_name_str) = extract_string(&iupac_name_value)
        {
            debug!("iupac_name: {iupac_name_str}");
            product.iupac_name = Some(iupac_name_str);
        }

        // InChi - safely extract if field exists
        if let Some(inchi_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='InChI')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(inchi_str) = extract_string(&inchi_value)
        {
            debug!("inchi: {inchi_str}");
            product.inchi = Some(inchi_str);
        }

        // InChi key - safely extract if field exists
        if let Some(inchi_key_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='InChIKey')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(inchi_key_str) = extract_string(&inchi_key_value)
        {
            debug!("inchi_key: {inchi_key_str}");
            product.inchi_key = Some(inchi_key_str);
        }

        // Canonical SMILES - safely extract if field exists
        if let Some(canonical_smiles_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='Canonical SMILES')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(canonical_smiles_str) = extract_string(&canonical_smiles_value)
        {
            debug!("canonical_smiles: {canonical_smiles_str}");
            product.canonical_smiles = Some(canonical_smiles_str);
        }

        // Molecular formula - safely extract if field exists
        if let Some(molecular_formula_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='Molecular Formula')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(molecular_formula_str) = extract_string(&molecular_formula_value)
        {
            debug!("molecular_formula: {molecular_formula_str}");
            product.molecular_formula = Some(molecular_formula_str);
        }

        // CAS - safely extract if field exists
        if let Some(cas_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='CAS')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(cas_str) = extract_string(&cas_value)
        {
            debug!("cas: {cas_str}");
            product.cas = Some(cas_str);
        }

        // EC - safely extract if field exists
        if let Some(ec_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='European Community (EC) Number')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(ec_str) = extract_string(&ec_value)
        {
            debug!("ec: {ec_str}");
            product.ec = Some(ec_str);
        }

        // Synonyms - safely extract if field exists
        if let Some(synonyms_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='Synonyms')].Section[?(@.TOCHeading!='Removed Synonyms')]..String",
        ) && let Some(synonym_str_vec) = extract_string_vec(&synonyms_value)
        {
            product.synonyms = Some(synonym_str_vec);
            // Sort and deduplicate synonyms
            if let Some(synonym_vec) = product.synonyms.as_mut() {
                synonym_vec.sort();
                synonym_vec.dedup();
            }
        }

        // Molecular weight - safely extract if field exists
        if let Some(molecular_weight_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.StringWithMarkup[0].String",
        ) && let Some(molecular_weight_str) = extract_string(&molecular_weight_value)
        {
            debug!("molecular_weight: {molecular_weight_str}");
            product.molecular_weight = Some(molecular_weight_str);
        }

        // Molecular weight unit - safely extract if field exists (separate query)
        if let Some(molecular_weight_unit_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.Unit",
        ) && let Some(molecular_weight_unit_str) = extract_string(&molecular_weight_unit_value)
        {
            debug!("molecular_weight_unit: {molecular_weight_unit_str}");
            product.molecular_weight_unit = Some(molecular_weight_unit_str);
        }

        // Boiling point - safely extract if field exists
        if let Some(boiling_point_value) = safe_json_query(
            &json_content,
            "$..Section[?(@.TOCHeading=='Boiling Point')].Information[?(@.Description=='PEER REVIEWED')].Value.StringWithMarkup[0].String",
        ) && let Some(boiling_point_str) = extract_string(&boiling_point_value)
        {
            debug!("boiling_point: {boiling_point_str}");
            product.boiling_point = Some(boiling_point_str);
        }

        // Symbols (pictograms) - safely extract if field exists
        if let Some(symbols_value) = safe_json_query(
            &json_content,
            "$..Information[?(@.Name=='Pictogram(s)')]..StringWithMarkup..Markup..URL",
        ) && let Some(symbol_str_vec) = extract_string_vec(&symbols_value)
        {
            let symbols_string = symbol_str_vec.join(",");
            let symbols_vec: Vec<String> = SYMBOL_RE
                .captures_iter(&symbols_string)
                .filter_map(|p| {
                    p.name("symbol")
                        .map(|statement| statement.as_str().to_string())
                })
                .collect();
            product.symbols = Some(symbols_vec);
            if let Some(symbol_vec) = product.symbols.as_mut() {
                symbol_vec.sort();
                symbol_vec.dedup();
            }
            debug!(
                "symbols: found {} symbols",
                product.symbols.as_ref().map_or(0, std::vec::Vec::len)
            );
        }

        // Signal - safely extract if field exists
        if let Some(signal_value) = safe_json_query(
            &json_content,
            "$..Information[?(@.Name=='Signal')]..StringWithMarkup..String",
        ) && let Some(signal_str_vec) = extract_string_vec(&signal_value)
        {
            product.signal = Some(signal_str_vec);
            if let Some(signal_vec) = product.signal.as_mut() {
                signal_vec.sort();
                signal_vec.dedup();
            }
            debug!(
                "signal: found {} signals",
                product.signal.as_ref().map_or(0, std::vec::Vec::len)
            );
        }

        // Hazard statements (GHS) - safely extract if field exists
        if let Some(hs_value) = safe_json_query(
            &json_content,
            "$..Information[?(@.Name=='GHS Hazard Statements')]..StringWithMarkup..String",
        ) && let Some(hs_str_vec) = extract_string_vec(&hs_value)
        {
            product.hs = Some(extract_safety_codes(&hs_str_vec, &PUBCHEM_HAZARD_CODE_RE));
            debug!(
                "hs: found {} hazard statements",
                product.hs.as_ref().map_or(0, std::vec::Vec::len)
            );
        }

        // Precautionary statements - safely extract if field exists
        if let Some(ps_value) = safe_json_query(
            &json_content,
            "$..Information[?(@.Name=='Precautionary Statement Codes')]..StringWithMarkup..String",
        ) && let Some(ps_str_vec) = extract_string_vec(&ps_value)
        {
            product.ps = Some(extract_safety_codes(
                &ps_str_vec,
                &PUBCHEM_PRECAUTIONARY_CODE_RE,
            ));
            debug!(
                "ps: found {} precautionary statements",
                product.ps.as_ref().map_or(0, std::vec::Vec::len)
            );
        }

        trace!("product parsed successfully: {product:#?}");

        product
    }
}

#[cfg(test)]
#[path = "pubchemproduct_tests.rs"]
mod pubchemproduct_tests;
