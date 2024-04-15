use jsonpath_rust::JsonPathQuery;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// A simplified pubchem product representation.
#[derive(Debug, Default, Serialize, Deserialize)]
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
    pub fn from_pubchem(json_content: String) -> Option<PubchemProduct> {
        // Precautionary statement regex.
        let precautionary_statement_re = Regex::new(r"(?P<statement>P[0-9]{3}\+{0,1})+").unwrap();
        // Hazard statement regex.
        let hazard_statement_re =
            Regex::new(r"(?P<statement>(AU|EU){0,1}H[0-9]{3}F{0,1}f{0,1}D{0,1}d{0,1}\+{0,1})+")
                .unwrap();
        // Symbol regex.
        let symbol_re = Regex::new(r"(?P<symbol>GHS0[1-9])").unwrap();

        // Final result.
        let mut product = PubchemProduct {
            ..Default::default()
        };

        let json_content: Value = serde_json::from_str(&json_content).unwrap();

        // Name.
        let name = json_content.clone().path("$.Record.RecordTitle").unwrap();

        debug!("name: {:#?}", name);
        let name_value = name.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.name = name_value.unwrap_or(None);

        // IUPAC name.
        let iupac_name = json_content.clone()
        .path("$..Section[?(@.TOCHeading=='IUPAC Name')].Information[0].Value.StringWithMarkup[0].String")
        .unwrap();

        debug!("iupac_name: {:#?}", iupac_name);
        let iupac_name_value = iupac_name.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.iupac_name = iupac_name_value.unwrap_or(None);

        // InChi.
        let inchi = json_content
            .clone()
            .path(
                "$..Section[?(@.TOCHeading=='InChI')].Information[0].Value.StringWithMarkup[0].String",
            )
            .unwrap();

        debug!("inchi: {:#?}", inchi);
        let inchi_value = inchi.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.inchi = inchi_value.unwrap_or(None);

        // InChi key.
        let inchi_key = json_content
        .clone()
        .path(
            "$..Section[?(@.TOCHeading=='InChIKey')].Information[0].Value.StringWithMarkup[0].String",
        )
        .unwrap();

        debug!("inchi_key: {:#?}", inchi_key);
        let inchi_key_value = inchi_key.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.inchi_key = inchi_key_value.unwrap_or(None);

        // Canonical SMILES.
        let canonical_smiles = json_content
        .clone()
        .path(
            "$..Section[?(@.TOCHeading=='Canonical SMILES')].Information[0].Value.StringWithMarkup[0].String",
        )
        .unwrap();

        debug!("canonical_smiles: {:#?}", canonical_smiles);
        let canonical_smiles_value = canonical_smiles.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.canonical_smiles = canonical_smiles_value.unwrap_or(None);

        // Molecular formula.
        let molecular_formula = json_content
        .clone()
        .path(
            "$..Section[?(@.TOCHeading=='Molecular Formula')].Information[0].Value.StringWithMarkup[0].String",
        )
        .unwrap();

        debug!("molecular_formula: {:#?}", molecular_formula);
        let molecular_formula_value = molecular_formula.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.molecular_formula = molecular_formula_value.unwrap_or(None);

        // CAS.
        let cas = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='CAS')].Information[0].Value.StringWithMarkup[0].String")
            .unwrap();

        debug!("cas: {:#?}", cas);
        let cas_value = cas.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.cas = cas_value.unwrap_or(None);

        // EC.
        let ec = json_content
        .clone()
        .path("$..Section[?(@.TOCHeading=='European Community (EC) Number')].Information[0].Value.StringWithMarkup[0].String")
        .unwrap();

        debug!("ec: {:#?}", ec);
        let ec_value = ec.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.ec = ec_value.unwrap_or(None);

        // Synonyms.
        let synonyms = json_content
            .clone()
            .path(
                "$..Section[?(@.TOCHeading=='Synonyms')].Section[?(@.TOCHeading!='Removed Synonyms')]..String",
            )
            .unwrap();

        debug!("synonyms: {:#?}", synonyms);
        product.synonyms = synonyms.as_array().map(|v| {
            v.iter()
                .map(|s| match s.as_str() {
                    Some(s) => s.to_string(),
                    None => "can not as_str".to_string(),
                })
                .collect()
        });

        if product.synonyms.is_some() {
            product.synonyms.as_mut().unwrap().sort();
            product.synonyms.as_mut().unwrap().dedup();
        }

        // Molecular weight.
        let molecular_weight = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.StringWithMarkup[0].String")
            .unwrap();

        debug!("molecular_weight: {:#?}", molecular_weight);
        let molecular_weight_value = molecular_weight.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.molecular_weight = molecular_weight_value.unwrap_or(None);

        // Molecular weight unit.
        let molecular_weight_unit = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.Unit")
            .unwrap();

        debug!("molecular_weight_unit: {:#?}", molecular_weight_unit);
        let molecular_weight_unit_value =
            molecular_weight_unit.as_array().map(|v| match v.get(0) {
                Some(value) => match value.as_str() {
                    Some(s) => Some(s.to_string()),
                    None => None,
                },
                None => None,
            });
        product.molecular_weight_unit = molecular_weight_unit_value.unwrap_or(None);

        // Boiling point.
        let boiling_point = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='Boiling Point')].Information[?(@.Description=='PEER REVIEWED')].Value.StringWithMarkup[0].String")
            .unwrap();

        debug!("boiling_point: {:#?}", boiling_point);
        let boiling_point_value = boiling_point.as_array().map(|v| match v.get(0) {
            Some(value) => match value.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        });
        product.boiling_point = boiling_point_value.unwrap_or(None);

        // Symbols.
        let symbols = json_content
            .clone()
            .path("$..Information[?(@.Name=='Pictogram(s)')]..StringWithMarkup..Markup..URL")
            .unwrap();

        debug!("symbols: {:#?}", symbols);
        let maybe_symbols_string_vec: Option<Vec<String>> = symbols.as_array().map(|v| {
            v.iter()
                .map(|s| match s.as_str() {
                    Some(s) => s.to_string(),
                    None => "can not as_str".to_string(),
                })
                .collect()
        });

        if let Some(symbols_string_vec) = maybe_symbols_string_vec {
            let symbols_string = symbols_string_vec.join(",");
            product.symbols = symbol_re
                .captures_iter(&symbols_string)
                .map(|p| {
                    p.name("symbol")
                        .map(|statement| statement.as_str().to_string())
                })
                .collect();
            product.symbols.as_mut().unwrap().sort();
            product.symbols.as_mut().unwrap().dedup();
        }

        // Signal.
        let signal = json_content
            .clone()
            .path("$..Information[?(@.Name=='Signal')]..StringWithMarkup..String")
            .unwrap();

        debug!("signal: {:#?}", signal);
        product.signal = signal.as_array().map(|v| {
            v.iter()
                .map(|s| match s.as_str() {
                    Some(s) => s.to_string(),
                    None => "can not as_str".to_string(),
                })
                .collect()
        });

        if product.signal.is_some() {
            product.signal.as_mut().unwrap().sort();
            product.signal.as_mut().unwrap().dedup();
        }

        // Hazard statements.
        let hs = json_content
            .clone()
            .path("$..Information[?(@.Name=='GHS Hazard Statements')]..StringWithMarkup..String")
            .unwrap();

        debug!("hs: {:#?}", hs);

        let maybe_hs_string_vec: Option<Vec<String>> = hs.as_array().map(|v| {
            v.iter()
                .map(|s| match s.as_str() {
                    Some(s) => s.to_string(),
                    None => "can not as_str".to_string(),
                })
                .collect()
        });

        if let Some(hs_string_vec) = maybe_hs_string_vec {
            let hs_string = hs_string_vec.join(",");
            product.hs = hazard_statement_re
                .captures_iter(&hs_string)
                .map(|p| {
                    p.name("statement")
                        .map(|statement| statement.as_str().to_string())
                })
                .collect();
            product.hs.as_mut().unwrap().sort();
            product.hs.as_mut().unwrap().dedup();
        }

        // Precautionary statements.
        let ps = json_content
            .clone()
            .path("$..Information[?(@.Name=='Precautionary Statement Codes')]..StringWithMarkup[0].String")
            .unwrap();

        debug!("ps: {:#?}", ps);

        let maybe_ps_string_vec: Option<Vec<String>> = ps.as_array().map(|v| {
            v.iter()
                .map(|s| match s.as_str() {
                    Some(s) => s.to_string(),
                    None => "can not as_str".to_string(),
                })
                .collect()
        });

        if let Some(ps_string_vec) = maybe_ps_string_vec {
            let ps_string = ps_string_vec.join(",");
            product.ps = precautionary_statement_re
                .captures_iter(&ps_string)
                .map(|p| {
                    p.name("statement")
                        .map(|statement| statement.as_str().to_string())
                })
                .collect();
            product.ps.as_mut().unwrap().sort();
            product.ps.as_mut().unwrap().dedup();
        }

        debug!("product: {:#?}", product);

        Some(product)
    }
}

#[cfg(test)]
mod tests {

    use std::{
        fs::{self},
        path::Path,
    };

    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        let json_file_path = Path::new("src/testdata/pubchem_pug_view.json");
        let json_string =
            fs::read_to_string(json_file_path).expect("error while opening json file");

        let product = PubchemProduct::from_pubchem(json_string);
        info!("{:#?}", product);
    }
}
