use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    ParseUnitTypeError,
    ParseProductTypeError,
    ParsePermissionNameError,
    ParsePermissionItemError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            ParseError::ParseUnitTypeError => write!(f, "wrong unit type"),
            ParseError::ParseProductTypeError => write!(f, "wrong product type"),
            ParseError::ParsePermissionNameError => write!(f, "wrong permission name"),
            ParseError::ParsePermissionItemError => write!(f, "wrong permission item"),
        }
    }
}

impl std::error::Error for ParseError {}
