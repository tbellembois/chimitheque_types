use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::ParseError;

/// Sort direction of a request, serialized as `"asc"` or `"desc"`.
///
/// Parsing (`FromStr` and JSON deserialization) is case-insensitive, so `"DESC"`
/// is accepted, but output (`Display` and JSON serialization) is always lowercase.
/// Any other value, including an empty string, is rejected.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase", try_from = "String")]
pub enum SortOrder {
    /// Ascending order (default).
    #[default]
    Asc,
    /// Descending order.
    Desc,
}

impl SortOrder {
    /// Returns the lowercase string representation: `"asc"` or `"desc"`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SortOrder {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<SortOrder, Self::Err> {
        if input.eq_ignore_ascii_case("asc") {
            Ok(SortOrder::Asc)
        } else if input.eq_ignore_ascii_case("desc") {
            Ok(SortOrder::Desc)
        } else {
            Err(ParseError::ParseSortOrderError(input.to_string()))
        }
    }
}

/// Used by serde to deserialize case-insensitively.
impl TryFrom<String> for SortOrder {
    type Error = ParseError;

    fn try_from(input: String) -> Result<SortOrder, Self::Error> {
        input.parse()
    }
}

#[cfg(test)]
#[path = "sortorder_tests.rs"]
mod sortorder_tests;
