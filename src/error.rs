use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    ParseUnitTypeError(String),
    ParseProductTypeError(String),
    ParsePermissionNameError(String),
    ParsePermissionItemError(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            ParseError::ParseUnitTypeError(s) => write!(f, "wrong unit type: {s}"),
            ParseError::ParseProductTypeError(s) => write!(f, "wrong product type: {s}"),
            ParseError::ParsePermissionNameError(s) => write!(f, "wrong permission name: {s}"),
            ParseError::ParsePermissionItemError(s) => write!(f, "wrong permission item: {s}"),
            ParseError::EmptyInput => write!(f, "empty input"),
        }
    }
}

impl std::error::Error for ParseError {}
