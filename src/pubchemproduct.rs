use crate::error::PubchemParseError;
use chimitheque_defines::SYMBOL_RE;
use jsonpath_rust::JsonPath;
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

fn json_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn query_strings(
    json: &Value,
    field: &'static str,
    query: &'static str,
) -> Result<Option<Vec<String>>, PubchemParseError> {
    let matches = json
        .query(query)
        .map_err(|error| PubchemParseError::Query {
            field,
            query,
            message: error.to_string(),
        })?;

    // No matches means absent optional data. A matched null is a type error.
    if matches.is_empty() {
        return Ok(None);
    }
    let strings = matches
        .into_iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or(PubchemParseError::InvalidType {
                    field,
                    expected: "string",
                    actual: json_type(value),
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(strings))
}

fn query_string(
    json: &Value,
    field: &'static str,
    query: &'static str,
) -> Result<Option<String>, PubchemParseError> {
    // Keep the existing first-match selection, but validate every matched value.
    Ok(query_strings(json, field, query)?.and_then(|values| values.into_iter().next()))
}

// A simplified pubchem product representation.
/// A simplified product extracted from a `PubChem` PUG View document.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PubchemProduct {
    /// Record title (common name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// IUPAC name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iupac_name: Option<String>,

    /// `InChI` identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inchi: Option<String>,

    /// `InChIKey` identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inchi_key: Option<String>,

    /// Canonical SMILES.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_smiles: Option<String>,

    /// Molecular formula.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_formula: Option<String>,

    /// CAS registry number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas: Option<String>,

    /// European Community (EC) number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec: Option<String>,

    /// Molecular weight, as text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weight: Option<String>,

    /// Unit of the molecular weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weight_unit: Option<String>,

    /// Peer-reviewed boiling point, as text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boiling_point: Option<String>,

    /// Synonyms (removed synonyms excluded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,

    /// URLs of the GHS pictograms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,

    /// GHS signal words.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<Vec<String>>,

    /// GHS hazard statement codes, e.g. `H350`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hs: Option<Vec<String>>,

    /// GHS precautionary statement codes, e.g. `P280`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<Vec<String>>,

    /// 2D structure picture, a base64-encoded PNG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twodpicture: Option<String>, // base64 encoded png
}

impl PubchemProduct {
    /// Extracts a product from a `PubChem` PUG View JSON document.
    ///
    /// Missing optional paths remain `None`. This extracts selected fields rather
    /// than validating the complete `PubChem` schema. Scalar fields retain the first
    /// match, but all matched values must be strings.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid JSON, a missing or non-object `Record`, an
    /// invalid `JSONPath` query, or a non-string extracted value (including null).
    ///
    /// # Examples
    ///
    /// ```
    /// use chimitheque_types::pubchemproduct::PubchemProduct;
    ///
    /// let product = PubchemProduct::from_pubchem_json(
    ///     r#"{"Record":{"RecordTitle":"Water"}}"#,
    /// )?;
    /// assert_eq!(product.name.as_deref(), Some("Water"));
    /// assert!(product.cas.is_none());
    /// # Ok::<(), chimitheque_types::error::PubchemParseError>(())
    /// ```
    pub fn from_pubchem_json(json_content: &str) -> Result<PubchemProduct, PubchemParseError> {
        // Final result.
        let mut product = PubchemProduct {
            ..Default::default()
        };

        // Do not turn malformed JSON or an API error response into an empty product.
        let json_content: Value = serde_json::from_str(json_content)?;
        if !json_content.is_object() {
            return Err(PubchemParseError::InvalidType {
                field: "$",
                expected: "object",
                actual: json_type(&json_content),
            });
        }
        match json_content.get("Record") {
            Some(Value::Object(_)) => {}
            record => {
                return Err(PubchemParseError::InvalidType {
                    field: "$.Record",
                    expected: "object",
                    actual: record.map_or("missing", json_type),
                });
            }
        }

        // Name - safely extract if field exists
        if let Some(name_str) = query_string(&json_content, "name", "$.Record.RecordTitle")? {
            debug!("name: {name_str}");
            product.name = Some(name_str);
        }

        // IUPAC name - safely extract if field exists
        if let Some(iupac_name_str) = query_string(
            &json_content,
            "iupac_name",
            "$..Section[?(@.TOCHeading=='IUPAC Name')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("iupac_name: {iupac_name_str}");
            product.iupac_name = Some(iupac_name_str);
        }

        // InChi - safely extract if field exists
        if let Some(inchi_str) = query_string(
            &json_content,
            "inchi",
            "$..Section[?(@.TOCHeading=='InChI')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("inchi: {inchi_str}");
            product.inchi = Some(inchi_str);
        }

        // InChi key - safely extract if field exists
        if let Some(inchi_key_str) = query_string(
            &json_content,
            "inchi_key",
            "$..Section[?(@.TOCHeading=='InChIKey')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("inchi_key: {inchi_key_str}");
            product.inchi_key = Some(inchi_key_str);
        }

        // Canonical SMILES - safely extract if field exists
        if let Some(canonical_smiles_str) = query_string(
            &json_content,
            "canonical_smiles",
            "$..Section[?(@.TOCHeading=='Canonical SMILES')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("canonical_smiles: {canonical_smiles_str}");
            product.canonical_smiles = Some(canonical_smiles_str);
        }

        // Molecular formula - safely extract if field exists
        if let Some(molecular_formula_str) = query_string(
            &json_content,
            "molecular_formula",
            "$..Section[?(@.TOCHeading=='Molecular Formula')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("molecular_formula: {molecular_formula_str}");
            product.molecular_formula = Some(molecular_formula_str);
        }

        // CAS - safely extract if field exists
        if let Some(cas_str) = query_string(
            &json_content,
            "cas",
            "$..Section[?(@.TOCHeading=='CAS')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("cas: {cas_str}");
            product.cas = Some(cas_str);
        }

        // EC - safely extract if field exists
        if let Some(ec_str) = query_string(
            &json_content,
            "ec",
            "$..Section[?(@.TOCHeading=='European Community (EC) Number')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("ec: {ec_str}");
            product.ec = Some(ec_str);
        }

        // Synonyms - safely extract if field exists
        if let Some(synonym_str_vec) = query_strings(
            &json_content,
            "synonyms",
            "$..Section[?(@.TOCHeading=='Synonyms')].Section[?(@.TOCHeading!='Removed Synonyms')]..String",
        )? {
            product.synonyms = Some(synonym_str_vec);
            // Sort and deduplicate synonyms
            if let Some(synonym_vec) = product.synonyms.as_mut() {
                synonym_vec.sort();
                synonym_vec.dedup();
            }
        }

        // Molecular weight - safely extract if field exists
        if let Some(molecular_weight_str) = query_string(
            &json_content,
            "molecular_weight",
            "$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.StringWithMarkup[0].String",
        )? {
            debug!("molecular_weight: {molecular_weight_str}");
            product.molecular_weight = Some(molecular_weight_str);
        }

        // Molecular weight unit - safely extract if field exists (separate query)
        if let Some(molecular_weight_unit_str) = query_string(
            &json_content,
            "molecular_weight_unit",
            "$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.Unit",
        )? {
            debug!("molecular_weight_unit: {molecular_weight_unit_str}");
            product.molecular_weight_unit = Some(molecular_weight_unit_str);
        }

        // Boiling point - safely extract if field exists
        if let Some(boiling_point_str) = query_string(
            &json_content,
            "boiling_point",
            "$..Section[?(@.TOCHeading=='Boiling Point')].Information[?(@.Description=='PEER REVIEWED')].Value.StringWithMarkup[0].String",
        )? {
            debug!("boiling_point: {boiling_point_str}");
            product.boiling_point = Some(boiling_point_str);
        }

        // Symbols (pictograms) - safely extract if field exists
        if let Some(symbol_str_vec) = query_strings(
            &json_content,
            "symbols",
            "$..Information[?(@.Name=='Pictogram(s)')]..StringWithMarkup..Markup..URL",
        )? {
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
        if let Some(signal_str_vec) = query_strings(
            &json_content,
            "signal",
            "$..Information[?(@.Name=='Signal')]..StringWithMarkup..String",
        )? {
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
        if let Some(hs_str_vec) = query_strings(
            &json_content,
            "hs",
            "$..Information[?(@.Name=='GHS Hazard Statements')]..StringWithMarkup..String",
        )? {
            product.hs = Some(extract_safety_codes(&hs_str_vec, &PUBCHEM_HAZARD_CODE_RE));
            debug!(
                "hs: found {} hazard statements",
                product.hs.as_ref().map_or(0, std::vec::Vec::len)
            );
        }

        // Precautionary statements - safely extract if field exists
        if let Some(ps_str_vec) = query_strings(
            &json_content,
            "ps",
            "$..Information[?(@.Name=='Precautionary Statement Codes')]..StringWithMarkup..String",
        )? {
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

        Ok(product)
    }
}

#[cfg(test)]
#[path = "pubchemproduct_tests.rs"]
mod pubchemproduct_tests;
