use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::{DiagnosticLevel, FileID};

mod for_std;

/// All result about notedown
pub type Result<T = ()> = std::result::Result<T, VomlError>;

/// Many errors
pub type Validation<T> = diagnostic::Validation<T, VomlError>;

/// Error type for all Notedown operators
#[derive(Debug)]
pub struct VomlError {
    /// Actual error kind
    pub kind: Box<VomlErrorKind>,
    /// Error level for report
    pub level: DiagnosticLevel,
    /// File name where error occurred
    pub file: FileID,
}

/// Actual error data for the error
#[derive(Debug)]
pub enum VomlErrorKind {
    /// The error type for I/O operations
    IOError(std::io::Error),
    /// Unknown error
    UnknownError,
}

impl Error for VomlError {}

impl Display for VomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for VomlErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
