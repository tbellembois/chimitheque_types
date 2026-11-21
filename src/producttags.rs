use serde::{Deserialize, Serialize};

/// A tag linked to a product, with its label.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Producttags {
    /// Id of the product.
    pub producttags_product_id: u64,
    /// Id of the tag.
    pub producttags_tag_id: u64,
    /// Label of the tag.
    pub producttags_tag_label: String,
}

#[cfg(test)]
#[path = "producttags_tests.rs"]
mod producttags_tests;
