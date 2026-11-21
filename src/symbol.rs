use crate::searchable::impl_searchable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Symbol {
    #[serde(default)]
    pub match_exact_search: bool,
    pub symbol_id: Option<u64>,
    pub symbol_label: String,
}

impl_searchable!(Symbol, table: "symbol", id: symbol_id, text: symbol_label);

#[cfg(test)]
#[path = "symbol_tests.rs"]
mod symbol_tests;
