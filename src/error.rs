use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    ParseUnitTypeError(String),
    ParseProductTypeError(String),
    ParsePermissionNameError(String),
    ParsePermissionItemError(String),
    ParseSortOrderError(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            ParseError::ParseUnitTypeError(s) => write!(f, "wrong unit type: {s}"),
            ParseError::ParseProductTypeError(s) => write!(f, "wrong product type: {s}"),
            ParseError::ParsePermissionNameError(s) => write!(f, "wrong permission name: {s}"),
            ParseError::ParsePermissionItemError(s) => write!(f, "wrong permission item: {s}"),
            ParseError::ParseSortOrderError(s) => write!(f, "wrong sort order: {s}"),
            ParseError::EmptyInput => write!(f, "empty input"),
        }
    }
}

impl std::error::Error for ParseError {}

/// An invalid `PubChem` document or a failure to extract one of its fields.
#[derive(Debug)]
#[non_exhaustive]
pub enum PubchemParseError {
    /// The input is not valid JSON. The original syntax error is preserved.
    InvalidJson(serde_json::Error),
    /// The document envelope or an extracted value has an unexpected JSON type.
    InvalidType {
        /// Product field name or document path where validation failed.
        field: &'static str,
        expected: &'static str,
        actual: &'static str,
    },
    /// A `JSONPath` expression could not be evaluated.
    Query {
        field: &'static str,
        query: &'static str,
        message: String,
    },
}

impl Display for PubchemParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJson(error) => write!(f, "invalid PubChem JSON: {error}"),
            Self::InvalidType {
                field,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "invalid PubChem field {field}: expected {expected}, found {actual}"
                )
            }
            Self::Query {
                field,
                query,
                message,
            } => {
                write!(f, "PubChem query failed for {field} ({query}): {message}")
            }
        }
    }
}

impl std::error::Error for PubchemParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidJson(error) => Some(error),
            Self::InvalidType { .. } | Self::Query { .. } => None,
        }
    }
}

impl From<serde_json::Error> for PubchemParseError {
    fn from(error: serde_json::Error) -> Self {
        Self::InvalidJson(error)
    }
}
