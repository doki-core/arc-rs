#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]

pub use diagnostic::{Diagnostic, DiagnosticLevel, FileID, Span, TextStorage};

pub use self::{
    error::{duplicate::DuplicateItem, Result, VError, VErrorKind, Validation},
    value::{Von, VonSerializer},
};

mod convert;
mod error;
mod for_3rd;
mod value;
