use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::{DiagnosticLevel, FileID, Span};

use crate::DuplicateItem;

mod for_std;

pub mod duplicate;

/// All result about notedown
pub type VResult<T = ()> = Result<T, VError>;

/// Many errors
pub type Validation<T> = diagnostic::Validation<T, VError>;

/// Error type for all Notedown operators
#[derive(Debug)]
pub struct VError {
    /// Actual error kind
    kind: Box<VErrorKind>,
    /// Error level for report
    level: DiagnosticLevel,

    source: Option<Box<dyn Error>>,
}

/// Actual error data for the error
pub enum VErrorKind {
    /// The error type for I/O operations
    IOError(std::io::Error),
    /// The error type for I/O operations
    ParseError(ParseFail),
    /// The error type for I/O operations
    Duplicate(DuplicateItem),
    /// Unknown error
    Custom(String),
}

#[derive(Debug)]
pub struct ParseFail {
    pub message: String,
    pub span: Span,
}

impl VError {
    #[inline]
    pub fn custom_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        VError { kind: Box::new(VErrorKind::Custom(message.into())), level: Default::default(), source: None }
    }
    #[inline]
    pub fn custom_result<T, S>(message: S) -> VResult<T>
    where
        S: Into<String>,
    {
        Err(VError::custom_error(message))
    }
}
