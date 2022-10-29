#![forbid(missing_docs)]
#![doc = include_str!("../Readme.md")]

pub use diagnostic::{Diagnostic, DiagnosticLevel, FileID, Span, TextStorage};

pub use self::error::{duplicate::DuplicateItem, Result, Validation, VomlError, VomlErrorKind};

mod error;
mod for_3rd;
