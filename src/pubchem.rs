use serde::{Deserialize, Serialize};

// Autocomplete
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutocompleteTerm {
    pub compound: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Autocomplete {
    pub total: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary_terms: Option<AutocompleteTerm>,
}
