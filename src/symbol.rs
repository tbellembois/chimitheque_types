use crate::searchable::impl_searchable;
use crate::{error::ValidationError, validation::require_text};
use chimitheque_utils::string::Transform;
use serde::{Deserialize, Serialize};

/// A GHS hazard pictogram, e.g. `GHS01`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Symbol {
    /// When searching, match the label exactly instead of partially.
    #[serde(default)]
    pub match_exact_search: bool,
    /// Database id; `None` for a new symbol.
    pub symbol_id: Option<u64>,
    /// Pictogram code, e.g. `GHS01`.
    pub symbol_label: String,
}

impl Symbol {
    /// Cleans the label and ensures it is not empty.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Empty` if the cleaned label is empty.
    pub fn sanitize_and_validate(&mut self) -> Result<(), ValidationError> {
        require_text(&mut self.symbol_label, "symbol_label", Transform::None)?;
        Ok(())
    }
}

impl_searchable!(Symbol, table: "symbol", id: symbol_id, text: symbol_label);

#[cfg(test)]
#[path = "symbol_tests.rs"]
mod symbol_tests;
