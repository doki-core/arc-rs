use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::DuplicateItem;
use diagnostic::{DiagnosticLevel, FileID, Span};

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
    pub kind: Box<VErrorKind>,
    /// Error level for report
    pub level: DiagnosticLevel,
    /// File name where error occurred
    pub file: FileID,
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
    UnknownError,
}

#[derive(Debug)]
pub struct ParseFail {
    pub message: String,
    pub span: Span,
}
