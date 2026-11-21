use serde::{Deserialize, Serialize};

// Autocomplete
/// Compound names suggested by the `PubChem` autocomplete service.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutocompleteTerm {
    /// Suggested compound names.
    pub compound: Vec<String>,
}

/// A `PubChem` autocomplete response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Autocomplete {
    /// Number of suggestions.
    pub total: usize,

    /// Suggested terms, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary_terms: Option<AutocompleteTerm>,
}

#[cfg(test)]
#[path = "pubchem_tests.rs"]
mod pubchem_tests;
