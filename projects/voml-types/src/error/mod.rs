use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use diagnostic::{DiagnosticLevel, Span};

use crate::VErrorKind::TypeMismatch;

mod for_std;

pub mod duplicate;

/// All result about notedown
pub type VResult<T = ()> = Result<T, VError>;

/// Many errors
pub type Validation<T> = diagnostic::Validation<T, VError>;

/// Error type for all Notedown operators

pub struct VError {
    /// Actual error kind
    pub kind: Box<VErrorKind>,
    /// Error level for report
    pub level: DiagnosticLevel,
}

/// Actual error data for the error
pub enum VErrorKind {
    /// The error type for I/O operations
    IOError {
        error: std::io::Error,
        file: String,
    },
    /// The error type for I/O operations
    ParseError {
        message: String,
        span: Span,
    },
    /// The error type for I/O operations
    Duplicate {
        /// kind
        kind: &'static str,
        /// item
        item: String,
        /// lsh
        lhs: Span,
        /// rhs
        rhs: Span,
    },
    TypeMismatch {
        expected: &'static str,
        actual: String,
    },
    /// Unknown error
    Custom(String),
}

impl VError {
    #[inline]
    pub fn custom_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        VError { kind: Box::new(VErrorKind::Custom(message.into())), level: Default::default() }
    }
    #[inline]
    pub fn type_mismatch<S>(expected: &str, actual: S) -> Self
    where
        S: Into<String>,
    {
        VError { kind: Box::new(TypeMismatch { expected, actual: actual.into() }), level: Default::default() }
    }
}
