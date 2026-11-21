use chimitheque_utils::{
    casnumber::CasNumberError, cenumber::CeNumberError, formula::ToEmpiricalFormulaError,
};
use std::fmt::{Display, Formatter};

/// A field rejected by a type's `sanitize_and_validate` method.
///
/// Messages are prefixed with the name of the field that failed, for example
/// `tag_label: empty input`.
#[derive(Debug)]
#[non_exhaustive]
pub enum ValidationError {
    /// A required field is empty after cleaning.
    Empty { field: &'static str },
    /// The CAS number label is not a valid CAS number.
    InvalidCasNumber(CasNumberError),
    /// The CE number label is not a valid CE number.
    InvalidCeNumber(CeNumberError),
    /// A formula field can not be converted into an empirical formula.
    InvalidFormula {
        field: &'static str,
        source: ToEmpiricalFormulaError,
    },
    /// The person email is not a valid mailbox address.
    InvalidEmail(email_address::Error),
    /// A number or date is outside its allowed range.
    OutOfRange {
        field: &'static str,
        /// Allowed range, e.g. `greater than 0`.
        expected: &'static str,
    },
    /// A text field does not have the expected format.
    InvalidFormat {
        field: &'static str,
        /// Expected format, e.g. `a valid InChIKey`.
        expected: &'static str,
    },
    /// A field is missing although another field requires it.
    Required {
        field: &'static str,
        /// Why the field is required, e.g. `when product_molecular_weight is set`.
        reason: &'static str,
    },
}

impl ValidationError {
    /// Returns the name of the field that failed validation.
    #[must_use]
    pub fn field(&self) -> &'static str {
        match self {
            Self::Empty { field }
            | Self::InvalidFormula { field, .. }
            | Self::OutOfRange { field, .. }
            | Self::InvalidFormat { field, .. }
            | Self::Required { field, .. } => field,
            Self::InvalidCasNumber(_) => "cas_number_label",
            Self::InvalidCeNumber(_) => "ce_number_label",
            Self::InvalidEmail(_) => "person_email",
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let field = self.field();
        match self {
            Self::Empty { .. } => write!(f, "{field}: empty input"),
            Self::InvalidCasNumber(error) => write!(f, "{field}: {error}"),
            Self::InvalidCeNumber(error) => write!(f, "{field}: {error}"),
            Self::InvalidFormula { source, .. } => write!(f, "{field}: {source}"),
            Self::InvalidEmail(error) => write!(f, "{field}: {error}"),
            Self::OutOfRange { expected, .. } | Self::InvalidFormat { expected, .. } => {
                write!(f, "{field}: must be {expected}")
            }
            Self::Required { reason, .. } => write!(f, "{field}: required {reason}"),
        }
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Empty { .. }
            | Self::OutOfRange { .. }
            | Self::InvalidFormat { .. }
            | Self::Required { .. } => None,
            Self::InvalidCasNumber(error) => Some(error),
            Self::InvalidCeNumber(error) => Some(error),
            Self::InvalidFormula { source, .. } => Some(source),
            Self::InvalidEmail(error) => Some(error),
        }
    }
}

/// An invalid request URL or query parameter in `RequestFilter::try_from`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestFilterError {
    /// The request is not a valid URL.
    InvalidUrl(url::ParseError),
    /// A query parameter value can not be parsed.
    InvalidParameter {
        parameter: &'static str,
        message: String,
    },
    /// A comma-separated list of ids is malformed.
    InvalidIdList { parameter: &'static str },
}

impl Display for RequestFilterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUrl(error) => write!(f, "can not parse url: {error}"),
            Self::InvalidParameter { parameter, message } => {
                write!(f, "error with {parameter} query parameter: {message}")
            }
            Self::InvalidIdList { parameter } => write!(f, "invalid {parameter} ids format"),
        }
    }
}

impl std::error::Error for RequestFilterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUrl(error) => Some(error),
            Self::InvalidParameter { .. } | Self::InvalidIdList { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
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

#[cfg(test)]
#[path = "error_tests.rs"]
mod error_tests;
