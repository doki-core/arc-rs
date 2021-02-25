use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Clone, Debug, PartialEq)]
pub enum ArcError {
    IOError(String),
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, ArcError>;
type PestError = arc_pest::Error<arc_pest::Rule>;
type IOError = std::io::Error;

impl Display for ArcError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ArcError {}

impl From<PestError> for ArcError {
    fn from(e: PestError) -> Self {
        Self::ParseError(format!("{:?}", e))
    }
}

impl From<IOError> for ArcError {
    fn from(e: IOError) -> Self {
        Self::IOError(format!("{:?}", e))
    }
}
