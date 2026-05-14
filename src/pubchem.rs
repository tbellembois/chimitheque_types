use serde::{Deserialize, Serialize};

// Autocomplete
#[derive(Serialize, Deserialize, Debug)]
pub struct AutocompleteTerm {
    compound: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Autocomplete {
    total: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary_terms: Option<AutocompleteTerm>,
}
