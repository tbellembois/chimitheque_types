use serde::{Deserialize, Serialize};

/// A hazard statement linked to a product, with its details.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producthazardstatements {
    /// Id of the product.
    pub producthazardstatements_product_id: u64,
    /// Id of the hazard statement.
    pub producthazardstatements_hazard_statement_id: u64,
    /// Label of the hazard statement.
    pub producthazardstatements_hazard_statement_label: String,
    /// Reference of the hazard statement.
    pub producthazardstatements_hazard_statement_reference: String,
    /// CMR classification of the hazard statement, if any.
    pub producthazardstatements_hazard_statement_cmr: Option<String>,
}

#[cfg(test)]
#[path = "producthazardstatements_tests.rs"]
mod producthazardstatements_tests;
