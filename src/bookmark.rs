use serde::{Deserialize, Serialize};

/// A product bookmarked by a person.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default)]
pub struct Bookmark {
    /// Database id; `None` for a new bookmark.
    pub bookmark_id: Option<u64>,
    /// Id of the person who bookmarked the product.
    pub person: u64,
    /// Id of the bookmarked product.
    pub product: u64,
}

#[cfg(test)]
#[path = "bookmark_tests.rs"]
mod bookmark_tests;
